use std::collections::HashMap;
use std::sync::{Arc, LockResult, RwLock};

use ehttp::{Mode, Request};
use lazy_static::lazy_static;
use log::info;

use shared::user::FrontAccount;

use crate::app::{ALL_ACCOUNTS, AUTH_INFO, AuthInfo};

pub const IS_DEBUG: bool = cfg!(debug_assertions);

pub fn get_oath2_url() -> String {
    let api_url: String = format!("{}api/oauth2/auth", get_api_path());
    api_url
}

pub fn authenticate() {
    if let Some(auth_id) = wasm_cookies::get("auth_id") {
        let api_url: String = format!("{}api/front/retrieve_auth_account?auth_id={}", get_api_path(), auth_id.unwrap());
        let mut request: Request = Request::get(api_url);
        request.mode = Mode::Cors;
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            let mut result = result.unwrap();
            if result.status == 200 {
                let account: FrontAccount = result.clone().json().unwrap();
                match AUTH_INFO.clone().write() {
                    Ok(mut lock) => {
                        lock.account = Option::from(account);
                        lock.authenticated = true;
                    }
                    Err(_) => {}
                };
                retrieve_accounts()
            }
        });
    }
}

pub fn retrieve_accounts() {
    let auth_id: String = wasm_cookies::get("auth_id").unwrap().unwrap();
    let api_url: String = format!("{}api/front/retrieve_accounts?auth_id={}", get_api_path(), auth_id);
    let mut request: Request = Request::get(api_url);

    request.mode = Mode::Cors;

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let mut result = result.unwrap();
        if result.status == 200 {
            let accounts: Vec<FrontAccount> = result.clone().json().unwrap();
            match ALL_ACCOUNTS.clone().write() {
                Ok(mut lock) => {
                    *lock = accounts;
                }
                Err(_) => {}
            };
        }
    });
}

pub fn get_api_path() -> String {
    let path: String = if IS_DEBUG {
        "http://localhost:2828/".to_string()
    } else {
        web_sys::window()
            .expect("no global `window` exists")
            .location().href().expect("should have a href").to_string()
    };
    path
}