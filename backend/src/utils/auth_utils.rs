use mongodb::bson::{doc, Document, to_bson};
use mongodb::Collection;
use oauth2::{RefreshToken, TokenResponse};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;
use reqwest::Response;

use shared::discord::DiscordAuthorizationInformation;
use shared::user::Account;

pub async fn is_auth_valid(user_token: &str, client: mongodb::Client) -> bool {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query = doc! {
        "token.access_token" : user_token
    };
    accounts.count_documents(query).await.unwrap() > 1
}

pub async fn update_account_discord(token: &str, client: mongodb::Client) {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query: Document = doc! {
        "token.access_token" : token
    };
    let account = accounts.find_one(query.clone()).await.expect("Can't find account to update").expect("Can't retrieve account document");

    let response: Response = reqwest::Client::new()
        .get("https://discord.com/api/oauth2/@me")
        .bearer_auth(token)
        .send()
        .await.expect("Can't get token_response");

    let authorization_information: DiscordAuthorizationInformation = response.json().await.expect("Can't parse authorization_information json");

    let update_doc = doc! {
        "$set": {
            "discord_user": to_bson(&authorization_information.user).unwrap()
        }
    };
    accounts.update_one(query, update_doc).await.expect("Failed to update account");
}

pub async fn renew_token(old_token: &str, renew_token: &RefreshToken, client: mongodb::Client, oauth_client: BasicClient) {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query: Document = doc! {
        "token.access_token" : old_token
    };
    accounts.find_one(query.clone()).await.expect("Can't find account to update").expect("Can't retrieve account document");

    let token_result: BasicTokenResponse = oauth_client
        .exchange_refresh_token(renew_token)
        .request_async(async_http_client)
        .await.expect("Can't renew token");

    let update_doc = doc! {
        "$set": {
            "token": to_bson(&token_result).unwrap(),
        }
    };
    accounts.update_one(query, update_doc).await.expect("Failed to update account for token change");
    update_account_discord(token_result.access_token().secret(), client).await;
}