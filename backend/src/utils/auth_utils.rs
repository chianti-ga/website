use std::time::{SystemTime, UNIX_EPOCH};

use log::info;
use mongodb::bson::{doc, Document, to_bson};
use mongodb::Collection;
use oauth2::{RefreshToken, TokenResponse};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;
use reqwest::{Client, Response};
use serde_json::Value;
use uuid::Uuid;
use shared::discord::{DiscordAuthorizationInformation, GuildMember};
use shared::user::Account;

use crate::CONFIG;

pub async fn is_auth_valid(auth_id: &str, client: mongodb::Client) -> bool {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query = doc! {
        "auth_id" : auth_id
    };
    accounts.count_documents(query).await.unwrap() > 0
}

pub async fn is_user_registered(discord_id: &String, client: mongodb::Client) -> bool {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query = doc! {
        "discord_user.id" : discord_id
    };
    accounts.count_documents(query).await.unwrap() > 0
}
pub async fn update_token(auth_id: &str, discord_id: &String, token_response: BasicTokenResponse, client: mongodb::Client, reqwest_client: &Client) {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query = doc! {
        "discord_user.id" : discord_id
    };
    let update_doc = doc! {
        "$set": {
            "token": to_bson(&token_response).unwrap(),
        }
    };
    accounts.update_one(query, update_doc).await.expect("Failed to update account for token change");
    update_account_discord(auth_id, client, reqwest_client).await;
}

pub async fn update_auth_id(discord_id: &String, auth_id: &String, client: mongodb::Client) {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query = doc! {
        "discord_user.id" : discord_id
    };
    let update_doc = doc! {
        "$set": {
            "auth_id": to_bson(&auth_id).unwrap(),
        }
    };
    accounts.update_one(query, update_doc).await.expect("Failed to update auth_id for account");
}

pub async fn update_account_discord(auth_id: &str, client: mongodb::Client, reqwest_client: &Client) {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query: Document = doc! {
        "auth_id" : auth_id
    };

    let account: Account = accounts.find_one(query.clone()).await.expect("Can't retrieve account to update").unwrap();
    let token: &String = account.token.access_token().secret();

    let response: Response = reqwest_client
        .get("https://discord.com/api/oauth2/@me")
        .bearer_auth(token)
        .send()
        .await.expect("Can't get token_response");

    let authorization_information: DiscordAuthorizationInformation = response.json().await.expect("Can't parse authorization_information json");

    let discord_guild_member_response: Response = reqwest_client
        .get("https://discord.com/api/users/@me/guilds/1031296063056924714/member")
        .bearer_auth(token)
        .send()
        .await.expect("Can't get token_response");

    let guild_member: GuildMember = discord_guild_member_response.json().await.unwrap_or(GuildMember {
        roles: vec![],
    });

    let update_doc = doc! {
        "$set": {
            "discord_user": to_bson(&authorization_information.user).unwrap(),
            "discord_roles" : to_bson(&guild_member.roles).unwrap()
        }
    };
    accounts.update_one(query, update_doc).await.expect("Failed to update account");
    info!("Discord info updated for {}({})",authorization_information.user.username, authorization_information.user.id);
}

pub async fn renew_token(old_token: &str, renew_token: &RefreshToken, client: mongodb::Client, oauth_client: BasicClient) {
    let accounts: Collection<Account> = client.database("visualis-website").collection("account");
    let query: Document = doc! {
        "token.access_token" : old_token
    };
    let account: Account = accounts.find_one(query.clone()).await.expect("Can't find account to update").expect("Can't retrieve account document");

    let token_result: BasicTokenResponse = oauth_client
        .exchange_refresh_token(renew_token)
        .request_async(async_http_client)
        .await.expect("Can't renew token");

    let time_now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("invalid time").as_secs();

    let auth_id: String = Uuid::now_v7().to_string();

    let update_doc = doc! {
        "$set": {
            "token": to_bson(&token_result).unwrap(),
            "last_renewal": to_bson(&time_now).unwrap(),
            "auth_id": to_bson(&auth_id).unwrap()
        }
    };

    accounts.update_one(query, update_doc).await.expect("Failed to update account for token change");
    update_account_discord(&account.auth_id, client, &Client::new()).await;
}