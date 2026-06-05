use chrono::Local;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::bulb::common::{Body, Command, Response};
use crate::settings::settings::{save_led_state, save_sign, save_time_stamp};

type HmacSha256 = Hmac<Sha256>;

/// Turns one or more Tuya smart bulbs on or off.
///
/// Sends a `switch_led` command to each device with the given `state_value`.
/// On success, persists the new state via [`save_led_state`]. Each request
/// is individually signed using HMAC-SHA256.
///
/// # Arguments
///
/// * `base_url` - Base URL of the Tuya Cloud API (e.g. `https://openapi.tuyaeu.com`)
/// * `client` - The `reqwest` HTTP client to use for requests
/// * `client_id` - Tuya API client ID
/// * `secret` - Tuya API secret, used for HMAC signing
/// * `access_token` - OAuth access token for the Tuya API
/// * `sign_method` - Signing method identifier sent in the request header
/// * `devices_ids` - Slice of device IDs to target
/// * `state_value` - `true` to turn the bulbs on, `false` to turn them off
///
/// # Errors
///
/// Does not return errors — request and parse failures are logged to stderr
/// via [`eprinln!`]. [`save_led_state`] is only called on success, so a
/// failed request will leave the persisted state unchanged.
///
/// # Panics
///
/// Panics if the request body cannot be serialized to JSON, or if the HMAC
/// key is invalid.
pub async fn bulb_state(
    base_url: &String,
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    access_token: &Option<String>,
    sign_method: &String,
    devices_ids: &[String],
    state_value: bool,
) {
    let client_id = client_id.as_deref().unwrap_or("");
    let secret = secret.as_deref().unwrap_or("");
    let access_token = access_token.as_deref().unwrap_or("");

    for (i, id) in devices_ids.iter().enumerate() {
        let t = Local::now().timestamp_millis().to_string();
        save_time_stamp(t.clone());

        let body = Body {
            commands: vec![Command {
                code: "switch_led".to_string(),
                value: state_value,
            }],
        };

        let body_json = serde_json::to_string(&body).expect("Failed to serialize body");

        let body_hash = format!("{:x}", Sha256::digest(body_json.as_bytes()));

        let path = format!("/v1.0/devices/{id}/commands");

        let content_to_sign = format!("POST\n{}\n\n{}", body_hash, path);

        let string_to_sign = format!("{}{}{}{}", client_id, access_token, t, content_to_sign);

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("Invalid HMAC key");

        mac.update(string_to_sign.as_bytes());

        let sign = hex::encode(mac.finalize().into_bytes()).to_uppercase();
        save_sign(&sign);

        let url = format!("{base_url}{path}");

        match client
            .post(url)
            .header("client_id", client_id)
            .header("access_token", access_token)
            .header("t", &t)
            .header("sign", &sign)
            .header("sign_method", sign_method)
            .json(&body)
            .send()
            .await
        {
            Ok(response) => match response.json::<Response>().await {
                Ok(_) => {
                    // println!("{:#?}", parsed);
                    save_led_state(state_value);
                    println!("Bulb {i}, changed status to {state_value}")
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
}
