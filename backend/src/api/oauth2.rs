use std::collections::HashMap;
use std::sync::{LockResult, Mutex};

use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::Redirect;
use log::error;
use oauth2::{AuthorizationCode, AuthUrl, Client, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RequestTokenError, Scope, StandardRevocableToken, TokenResponse, TokenUrl};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use oauth2::reqwest::{async_http_client, http_client};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppData, CONFIG};
use crate::utils::config_utils::Oauth2Client;

#[derive(Deserialize, Debug, Clone)]
struct OAuth2Callback {
    pub code: String,
    pub state: String,
}
#[get("/api/oauth2/auth/")]
pub async fn auth(session: Session, app_data: web::Data<AppData>) -> impl Responder {
    let oauth2_info: &Oauth2Client = &CONFIG.oauth2client.clone();
    //IMPORTANT: The urls should have "/" appended to the end, the lib will crash if so
    let client =
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
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let client_id = Uuid::now_v7();
    session.insert("client_id", client_id.clone()).expect("TODO: panic message");
    app_data.client_map.lock().unwrap().insert(client_id.to_string(), client);
    session.insert("pkce_verif", pkce_verifier.secret()).expect("TODO: panic message");

    Redirect::to(auth_url.to_string())
}

#[get("/api/oauth2/callback")]
pub async fn callback(callback_data: web::Query<OAuth2Callback>, session: Session, app_data: web::Data<AppData>) -> impl Responder {
    println!("{:?}", session.entries());
    return match app_data.client_map.lock() {
        Ok(hashmap) => {
            let client_id = session.get::<String>("client_id").unwrap().unwrap();
            let client = hashmap.get(&client_id).unwrap();
            let pkce_verifier: String = session.get::<String>("pkce_verif").unwrap().unwrap();

            let code = callback_data.code.clone();

            match client.exchange_code(AuthorizationCode::new(code))
                .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
                .request_async(async_http_client).await {
                Ok(token_response) => {
                    HttpResponse::Ok().body(format!("{:?}\n{:?}\n{:?}", callback_data.0, client_id, token_response))
                }
                Err(err) => {
                    eprintln!("{}", err);
                    HttpResponse::InternalServerError().body("")
                }
            }
        }
        Err(err) => {
            error!("{}",err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    };
}