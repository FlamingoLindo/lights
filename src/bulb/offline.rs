use crate::settings::settings::{save_sign, save_time_stamp};
use chrono::Local;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize)]
struct DeviceResponse {
    success: bool,
    result: Option<DeviceResult>,
}

#[derive(Deserialize)]
struct DeviceResult {
    online: bool,
}

pub async fn online_devices(
    base_url: &String,
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    access_token: &Option<String>,
    sign_method: &String,
    devices_ids: &[String],
) -> Vec<String> {
    let client_id = client_id.as_deref().unwrap_or("");
    let secret = secret.as_deref().unwrap_or("");
    let access_token = access_token.as_deref().unwrap_or("");
    let mut online: Vec<String> = Vec::new();

    let empty_hash = format!("{:x}", Sha256::digest(b""));

    for (i, id) in devices_ids.iter().enumerate() {
        let t = Local::now().timestamp_millis().to_string();
        save_time_stamp(t.clone());

        let path = format!("/v1.0/devices/{id}");
        let content_to_sign = format!("GET\n{}\n\n{}", empty_hash, path);
        let string_to_sign = format!("{}{}{}{}", client_id, access_token, t, content_to_sign);

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("Invalid HMAC key");
        mac.update(string_to_sign.as_bytes());
        let sign = hex::encode(mac.finalize().into_bytes()).to_uppercase();
        save_sign(&sign);

        let url = format!("{base_url}{path}");

        match client
            .get(url)
            .header("client_id", client_id)
            .header("access_token", access_token)
            .header("t", &t)
            .header("sign", &sign)
            .header("sign_method", sign_method)
            .send()
            .await
        {
            Ok(response) => match response.json::<DeviceResponse>().await {
                Ok(parsed) => {
                    if parsed.success && parsed.result.map(|r| r.online).unwrap_or(false) {
                        println!("Device {id} is online");
                        online.push(id.clone());
                    } else {
                        // TODO if offline keep trying until it connects?
                        println!("Device {i} is offline, skipping");
                    }
                }
                Err(err) => eprintln!("Failed to parse response for {id}: {err}"),
            },
            Err(err) => eprintln!("Request failed for {id}: {err}"),
        }
    }

    online
}
