use chrono::Local;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::bulb::common::{Body, Command, Response};
use crate::settings::settings::{save_led_state, save_sign, save_time_stamp};

type HmacSha256 = Hmac<Sha256>;

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
