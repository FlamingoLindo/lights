use crate::settings::settings::{save_sign, save_time_stamp, save_token};
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    result: Result,
    success: bool,
    t: i64,
    tid: String,
}

#[derive(Serialize, Deserialize)]
struct Result {
    access_token: String,
    expire_time: i64,
    refresh_token: String,
    uid: String,
}

pub async fn request_tuya_token(
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    base_url: &String,
    expires_at: &Option<i64>,
) {
    let now = Local::now().timestamp();

    if let Some(exp) = expires_at {
        if now < *exp {
            println!("Token still valid, skipping request.");
            return;
        }
    }

    let t: String = Local::now().timestamp_millis().to_string();
    save_time_stamp(t.clone());

    let client_id = client_id.as_deref().unwrap_or("");
    let secret = secret.as_deref().unwrap_or("");

    let mut hasher = Sha256::new();
    hasher.update(b"");
    let body_hash = format!("{:x}", hasher.finalize());

    let content_to_sign = format!("GET\n{body_hash}\n\n/v1.0/token?grant_type=1");
    let string_to_sign = format!("{client_id}{t}{content_to_sign}");

    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let sign = hex::encode(mac.finalize().into_bytes()).to_uppercase();

    save_sign(&sign);

    let url = format!("{base_url}/v1.0/token?grant_type=1");

    match client
        .get(url)
        .header("client_id", client_id)
        .header("sign_method", "HMAC-SHA256")
        .header("t", &t)
        .header("sign", &sign)
        .header("access_token", "")
        .send()
        .await
    {
        Ok(response) => match response.json::<TokenResponse>().await {
            Ok(parsed) => {
                save_token(
                    parsed.result.access_token,
                    parsed.result.refresh_token,
                    parsed.result.expire_time,
                );
            }
            Err(err) => {
                eprintln!("Failed to parse response: {err}");
            }
        },
        Err(err) => {
            eprintln!("Request failed: {err}");
        }
    }
}
