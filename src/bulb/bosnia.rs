use crate::bulb::common::{Body, Command, CommandValue, Response};
use crate::settings::settings::save_sign;
use crate::{bulb::state::bulb_state, settings::settings::save_time_stamp};
use chrono::Local;
use hmac::{Hmac, Mac};
use palette::{Hsv, IntoColor, Srgb, named};
use sha2::{Digest, Sha256};
type HmacSha256 = Hmac<Sha256>;

pub async fn bosnian_bulbs(
    base_url: &String,
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    devices_ids: &[String],
    sign_method: &String,
    access_token: &Option<String>,
) {
    // Turn bulbs on in case they were previously off
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

    let client_id_value = client_id.as_deref().unwrap_or("");
    let secret_value = secret.as_deref().unwrap_or("");
    let access_token_value = access_token.as_deref().unwrap_or("");

    let blue = Srgb::<f32>::from_format(named::BLUE);
    let yellow = Srgb::<f32>::from_format(named::YELLOW);

    let blue_hsv: Hsv = blue.into_color();
    let yellow_hsv: Hsv = yellow.into_color();

    let colors = [blue_hsv, yellow_hsv];

    for (i, id) in devices_ids.iter().enumerate() {
        let hsv = colors[i % colors.len()];

        let t = Local::now().timestamp_millis().to_string();
        save_time_stamp(t.clone());

        let h = hsv.hue.into_degrees().rem_euclid(360.0).round() as i32;
        let s = (hsv.saturation * 1000.0).round() as i32;
        let v = (hsv.value * 1000.0).round() as i32;

        let body = Body {
            commands: vec![
                Command {
                    code: "work_mode".to_string(),
                    value: CommandValue::Str("colour".to_string()),
                },
                Command {
                    code: "colour_data_v2".to_string(),
                    value: CommandValue::Map(serde_json::json!({
                        "h": h,
                        "s": s,
                        "v": v
                    })),
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
            .body(body_json)
            .send()
            .await
        {
            Ok(response) => match response.json::<Response>().await {
                Ok(_) => {
                    println!("Bosnia: Bulb {i} color changed")
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
