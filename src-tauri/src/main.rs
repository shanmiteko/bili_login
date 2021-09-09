#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod client;
mod codec;
mod http;

use std::ops::Add;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri::command;
use tokio::sync::Mutex;

use client::{CaptchaCombine, LoginInfo, LoginKeys};
use codec::rsa_encode;

lazy_static! {
    static ref LOGIN_INFO: Mutex<LoginInfo> = Mutex::new(LoginInfo::new());
    static ref GT: Mutex<String> = Mutex::new(String::new());
}

#[derive(Deserialize)]
struct Account {
    uname: String,
    pwd: String,
}

#[derive(Deserialize)]
struct GeetestResult {
    validate: String,
    seccode: String,
}

#[derive(Serialize)]
struct GeetestRequest {
    gt: String,
    challenge: String,
}

#[command]
async fn send_login_info(Account { uname, pwd }: Account) -> GeetestRequest {
    LOGIN_INFO.lock().await.username(uname);
    match LoginKeys::fetch().await {
        Ok(LoginKeys { hash, key }) => {
            LOGIN_INFO
                .lock()
                .await
                .password(rsa_encode(&key, &hash.add(&pwd)));
        }
        Err(error) => {
            panic!("{}", error)
        }
    }

    GeetestRequest {
        gt: GT.lock().await.clone(),
        challenge: LOGIN_INFO.lock().await.challenge.clone(),
    }
}

#[command]
async fn send_geetest_result(GeetestResult { validate, seccode }: GeetestResult) -> String {
    LOGIN_INFO.lock().await.validate(validate);
    LOGIN_INFO.lock().await.seccode(seccode);

    match LOGIN_INFO.lock().await.fetch().await {
        Ok(m) => m,
        Err(e) => e.to_string(),
    }
}

#[command]
async fn update_login_info() {
    *LOGIN_INFO.lock().await = LoginInfo::new();
    set_global().await;
}

async fn set_global() {
    match CaptchaCombine::fetch().await {
        Ok(CaptchaCombine { gt, challenge, key }) => {
            LOGIN_INFO.lock().await.challenge(challenge);
            LOGIN_INFO.lock().await.key(key);

            *GT.lock().await = gt;
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

#[tokio::main]
async fn main() {
    set_global().await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_login_info,
            send_geetest_result,
            update_login_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
