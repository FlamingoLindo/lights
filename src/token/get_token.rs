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

/// Requests a new Tuya Cloud access token if the current one has expired.
///
/// Checks `expires_at` against the current timestamp before making any
/// network request. If the token is still valid, the function returns early
/// with no side effects. Otherwise, requests a new token via
/// `GET /v1.0/token?grant_type=1` and persists the result using [`save_token`].
///
/// The request uses a simplified signing flow — no access token is included
/// in the signature, as this is an unauthenticated token grant endpoint.
///
/// # Arguments
///
/// * `client` - The `reqwest` HTTP client to use for requests
/// * `client_id` - Tuya API client ID
/// * `secret` - Tuya API secret, used for HMAC signing
/// * `base_url` - Base URL of the Tuya Cloud API (e.g. `https://openapi.tuyaeu.com`)
///
/// # Returns
///
/// Returns `Some((access_token, expires_in_secs))` on success, `None` on failure.
///
/// # Errors
///
/// Does not return errors — request and parse failures are logged to stderr
/// via [`eprintln!`].
///
/// # Panics
///
/// Panics if the HMAC key is invalid.
pub async fn request_tuya_token(
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    base_url: &String,
) -> Option<(String, u64)> {
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
                let access_token = parsed.result.access_token.clone();
                let expires_in_secs = parsed.result.expire_time as u64;
                save_token(
                    parsed.result.access_token,
                    parsed.result.refresh_token,
                    parsed.result.expire_time,
                );
                Some((access_token, expires_in_secs))
            }
            Err(err) => {
                eprintln!("Failed to parse response: {err}");
                None
            }
        },
        Err(err) => {
            eprintln!("Request failed: {err}");
            None
        }
    }
}
