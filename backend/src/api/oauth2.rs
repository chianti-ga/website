use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::Redirect;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::BasicClient;
use serde::Deserialize;

use crate::CONFIG;

#[derive(Deserialize, Debug)]
struct OAuth2Callback {
    code: String,
    state: String,
}
#[get("/api/oauth2/auth/")]
pub async fn auth() -> impl Responder {
    let oauth2_info = &CONFIG.oauth2client.clone();
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
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    Redirect::to(auth_url.to_string())
}

#[get("/api/oauth2/callback")]
pub async fn callback(callback: web::Query<OAuth2Callback>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", callback.0))
}