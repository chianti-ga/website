use std::collections::HashMap;
use std::sync::{Arc, LockResult, RwLock};

use ehttp::{Headers, Mode, Request};
use lazy_static::lazy_static;
use log::info;

use shared::fiche_rp::{FicheRP, ReviewMessage};
use shared::user::FrontAccount;

use crate::app::{ALL_ACCOUNTS, AUTH_INFO, AuthInfo};

pub const IS_DEBUG: bool = cfg!(debug_assertions);

pub fn get_oath2_url() -> String {
    let api_url: String = format!("{}api/oauth2/auth", get_api_path());
    api_url
}
// can also be used to update user info
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

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let mut result = result.unwrap();
        info!("{}", &result.text().unwrap());
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

pub fn post_ficherp(ficherp: &FicheRP) {
    let auth_id: String = wasm_cookies::get("auth_id").unwrap().unwrap();
    let api_url: String = format!("{}api/front/submit_ficherp?auth_id={}", get_api_path(), auth_id);
    let request: Request = post_json(api_url, serde_json::to_string(ficherp).unwrap().into_bytes());

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let mut result = result.unwrap();
        info!("{}", &result.text().unwrap());

        if result.status == 200 {
            retrieve_accounts();
            authenticate();
        }
    });
}

pub fn post_comment(comment: &ReviewMessage, ficherp_id: String) {
    let auth_id: String = wasm_cookies::get("auth_id").unwrap().unwrap();
    let api_url: String = format!("{}api/front/submit_comment?auth_id={}&fiche_id={}", get_api_path(), auth_id, ficherp_id);
    let request: Request = post_json(api_url, serde_json::to_string(comment).unwrap().into_bytes());

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let mut result = result.unwrap();
        info!("{}", &result.text().unwrap());

        if result.status == 200 {
            retrieve_accounts();
            authenticate();
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

fn post_json(url: String, body: Vec<u8>) -> Request {
    Request {
        method: "POST".to_owned(),
        url: url,
        body,
        headers: Headers::new(&[
            ("Accept", "*/*"),
            ("Content-Type", "application/json; charset=utf-8"),
        ]),
        #[cfg(target_arch = "wasm32")]
        mode: Mode::default(),
    }
}