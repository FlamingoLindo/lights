use chrono::Local;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::bulb::common::{Body, Command, CommandValue, Response};
use crate::{
    bulb::state::bulb_state,
    settings::settings::{DefaultSettings, save_sign, save_time_stamp},
};

type HmacSha256 = Hmac<Sha256>;

pub async fn bulb_default(
    base_url: &String,
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    devices_ids: &Vec<String>,
    sign_method: &String,
    access_token: &Option<String>,
    default_settings: &DefaultSettings,
) {
    let client_id_value = client_id.as_deref().unwrap_or("");
    let secret_value = secret.as_deref().unwrap_or("");
    let access_token_value = access_token.as_deref().unwrap_or("");

    for (i, id) in devices_ids.iter().enumerate() {
        let t = Local::now().timestamp_millis().to_string();
        save_time_stamp(t.clone());

        let body = Body {
            commands: vec![
                Command {
                    code: "work_mode".to_string(),
                    value: CommandValue::Str(default_settings.work_mode.to_string()),
                },
                Command {
                    code: "bright_value_v2".to_string(),
                    value: CommandValue::Int(default_settings.bright_value_v2),
                },
                Command {
                    code: "temp_value_v2".to_string(),
                    value: CommandValue::Int(default_settings.temp_value_v2),
                },
            ],
        };

        let body_json = serde_json::to_string(&body).expect("Failed to serialize body");

        let body_hash = format!("{:x}", Sha256::digest(body_json.as_bytes()));

        let path = format!("/v1.0/devices/{id}/commands");

        let content_to_sign = format!("POST\n{}\n\n{}", body_hash, path);

        let string_to_sign = format!(
            "{}{}{}{}",
            client_id_value, access_token_value, t, content_to_sign
        );

        let mut mac =
            HmacSha256::new_from_slice(secret_value.as_bytes()).expect("Invalid HMAC key");

        mac.update(string_to_sign.as_bytes());

        let sign = hex::encode(mac.finalize().into_bytes()).to_uppercase();
        save_sign(&sign);

        let url = format!("{base_url}{path}");

        match client
            .post(url)
            .header("client_id", client_id_value)
            .header("access_token", access_token_value)
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
                    println!("Bulb {i} back to default")
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

    // Turn on the bulbs again
    bulb_state(
        base_url,
        client,
        client_id,
        secret,
        access_token,
        sign_method,
        devices_ids,
        true,
    )
    .await;
}
