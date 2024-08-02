use std::time::{SystemTime, UNIX_EPOCH};

use actix_session::Session;
use actix_web::{get, HttpRequest, Responder, web};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::http::header;
use actix_web::web::Redirect;
use log::{error, info};
use mongodb::bson::doc;
use mongodb::Collection;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use shared::discord::{DiscordAuthorizationInformation, GuildMember};
use shared::user::Account;

use crate::{AppData, CONFIG};
use crate::utils::auth_utils::{is_auth_valid, is_user_registered, update_account_discord, update_auth_id, update_token};
use crate::utils::config_utils::Oauth2Client;

#[derive(Deserialize, Debug, Clone)]
struct OAuth2Callback {
    pub code: String,
    pub state: String,
}
#[get("/api/oauth2/auth/")]
pub async fn auth(req: HttpRequest, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    if let Some(cookie) = req.cookie("auth_id") {
        if is_auth_valid(cookie.value(), app_data.dbclient.clone()).await {
            update_account_discord(cookie.value(), app_data.dbclient.clone()).await;
            return actix_web::HttpResponse::Found()
                .append_header((header::LOCATION, "/"))
                .finish();
        }
    }
    let oauth2_info: &Oauth2Client = &CONFIG.oauth2client.clone();
    //IMPORTANT: The urls should NOT have "/" appended to the end, the lib will crash if so
    let oauth_client =
        BasicClient::new(
            ClientId::new(oauth2_info.client_id.clone()),
            Some(ClientSecret::new(oauth2_info.client_secret.clone())),
            AuthUrl::new(oauth2_info.auth_url.clone()).unwrap(),
            Some(TokenUrl::new(oauth2_info.token_url.clone()).unwrap()))
            // Set the URL the user will be redirected to after the authorization process.
            .set_redirect_uri(RedirectUrl::new(oauth2_info.redirect_url.clone()).unwrap());

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()))
        .add_scope(Scope::new("guilds.members.read".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let client_id = Uuid::now_v7();
    session.insert("client_id", client_id).expect("TODO: panic message");
    app_data.client_map.insert(client_id.to_string(), oauth_client);
    session.insert("pkce_verif", pkce_verifier.secret()).expect("TODO: panic message");

    actix_web::HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish()
}

#[get("/api/oauth2/callback")]
pub async fn callback(callback_data: web::Query<OAuth2Callback>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    let mut client_id_value: String = String::new();
    let mut pkce_verifier_value: String = String::new();

    if let (Ok(Some(client_id)), Ok(Some(pkce_verifier))) = (session.get::<String>("client_id"), session.get::<String>("pkce_verif")) {
        client_id_value = client_id;
        pkce_verifier_value = pkce_verifier;
    } else {
        return actix_web::HttpResponse::BadRequest().body("bad request");
    }

    let client = app_data.client_map.get(&client_id_value).unwrap();
    let code: String = callback_data.code.clone();

    match client.value().exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier_value))
        .request_async(async_http_client).await {
        Ok(token_response) => {
            let discord_autho_response: Response = reqwest::Client::new()
                .get("https://discord.com/api/oauth2/@me")
                .bearer_auth(token_response.clone().access_token().secret())
                .send()
                .await.expect("Can't get token_response");

            let accounts: Collection<Account> = app_data.dbclient.database("visualis-website").collection("account");
            let authorization_information: DiscordAuthorizationInformation = discord_autho_response.json().await.expect("Can't parse authorization_information json");

            if is_user_registered(&authorization_information.user.id, app_data.dbclient.clone()).await {
                info!("User already registered, updating token...");

                let auth_id: String = Uuid::now_v7().to_string();

                update_auth_id(&authorization_information.user.id, &auth_id, app_data.dbclient.clone()).await;

                let mut auth_cookie: Cookie = Cookie::new("auth_id", &auth_id);
                auth_cookie.set_secure(true);
                auth_cookie.set_domain(&CONFIG.domain);
                auth_cookie.set_path("/");
                auth_cookie.set_same_site(SameSite::Strict);

                update_token(&auth_id, &authorization_information.user.id, token_response.clone(), app_data.dbclient.clone()).await;
                info!("Token updated for {}({})",authorization_information.user.username.clone(), authorization_information.user.id.clone());

                return actix_web::HttpResponse::Ok().cookie(auth_cookie).body("");
            }

            let discord_guild_member_response: Response = reqwest::Client::new()
                .get("https://discord.com/api/users/@me/guilds/1031296063056924714/member")
                .bearer_auth(token_response.clone().access_token().secret())
                .send()
                .await.expect("Can't get token_response");

            let member_json: Value = discord_guild_member_response.json().await.expect("Can't json value");
            println!("{}", member_json);

            if let Some(code) = member_json.get("code") {
                return if code.to_string().contains("10004") {
                    actix_web::HttpResponse::Found().append_header((header::LOCATION, "https://discord.gg/PPAeJbQacn")).finish()
                } else {
                    actix_web::HttpResponse::UnprocessableEntity().body(member_json.to_string())
                };
            }

            let guild_member: GuildMember = serde_json::from_value(member_json).unwrap();

            let time_now: u64 = SystemTime::now()
                .duration_since(UNIX_EPOCH).expect("invalid time")
                .as_secs();

            let auth_id: String = Uuid::now_v7().to_string();

            let authenticated_user = Account {
                discord_user: authorization_information.user,
                discord_roles: guild_member.roles,
                auth_id: auth_id.clone(),
                token: token_response.clone(),
                last_renewal: time_now,
                fiches: vec![],
                creation_date: time_now,
                banned: false,
            };
            accounts.insert_one(authenticated_user).await.expect("Can't insert new user");

            let mut auth_cookie: Cookie = Cookie::new("auth_id", auth_id);
            auth_cookie.set_secure(false);
            auth_cookie.set_domain(&CONFIG.domain);
            auth_cookie.set_same_site(SameSite::Strict);
            auth_cookie.set_path("/");
            auth_cookie.set_expires(OffsetDateTime::now_utc() + Duration::weeks(2));

            actix_web::HttpResponse::Found()
                .append_header((header::LOCATION, "/"))
                .cookie(auth_cookie)
                .finish()
        }
        Err(err) => {
            error!("{}", err);
            actix_web::HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
