use std::error::Error;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use actix_session::Session;
use actix_web::{get, HttpRequest, Responder, web};
use actix_web::cookie::Cookie;
use actix_web::web::Redirect;
use log::error;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::results::InsertOneResult;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;
use reqwest::Response;
use serde::Deserialize;
use uuid::Uuid;

use shared::discord::DiscordAuthorizationInformation;
use shared::user::Account;

use crate::{AppData, CONFIG};
use crate::utils::auth_utils::{is_auth_valid, update_account_discord};
use crate::utils::config_utils::Oauth2Client;

#[derive(Deserialize, Debug, Clone)]
struct OAuth2Callback {
    pub code: String,
    pub state: String,
}
#[get("/api/oauth2/auth/")]
pub async fn auth(req: HttpRequest, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        if is_auth_valid(cookie.value().clone(), app_data.dbclient.clone()).await {
            update_account_discord(cookie.value(), app_data.dbclient.clone()).await;
            return Redirect::to("http://localhost:8080");
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
            .set_redirect_uri(RedirectUrl::new("http://localhost:8080/api/oauth2/callback".to_string()).unwrap());

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let client_id = Uuid::now_v7();
    session.insert("client_id", client_id.clone()).expect("TODO: panic message");
    app_data.client_map.insert(client_id.to_string(), oauth_client);
    session.insert("pkce_verif", pkce_verifier.secret()).expect("TODO: panic message");

    Redirect::to(auth_url.to_string())
}

#[get("/api/oauth2/callback")]
pub async fn callback(req: HttpRequest, callback_data: web::Query<OAuth2Callback>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    let client_id: String = session.get::<String>("client_id").unwrap().unwrap();
    let client = app_data.client_map.get(&client_id).unwrap();
    let pkce_verifier: String = session.get::<String>("pkce_verif").unwrap().unwrap();
    let code: String = callback_data.code.clone();

    match client.value().exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client).await {
        Ok(token_response) => {
            let response: Response = reqwest::Client::new()
                .get("https://discord.com/api/oauth2/@me")
                .bearer_auth(token_response.clone().access_token().secret())
                .send()
                .await.expect("Can't get token_response");

            let accounts: Collection<Account> = app_data.dbclient.database("visualis-website").collection("account");
            let authorization_information: DiscordAuthorizationInformation = response.json().await.expect("Can't parse authorization_information json");

            let time_now: u64 = SystemTime::now()
                .duration_since(UNIX_EPOCH).expect("invalid time")
                .as_secs();

            let authenticated_user = Account {
                discord_user: authorization_information.user,
                token: token_response.clone(),
                last_renewal: 0,
                fiches: vec![],
                creation_date: time_now,
                banned: false,
            };
            accounts.insert_one(authenticated_user).await.expect("Can't insert new user");

            let mut token_cookie: Cookie = Cookie::new("token", token_response.access_token().secret());
            token_cookie.set_secure(true);

            actix_web::HttpResponse::Ok().cookie(token_cookie).body("")
        }
        Err(err) => {
            error!("{}", err);
            actix_web::HttpResponse::InternalServerError().body(format!("{}", err.to_string()))
        }
    }
}
