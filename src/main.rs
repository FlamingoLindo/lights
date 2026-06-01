mod bulb;
mod settings;
mod token;
mod tuya_client;

use crate::settings::settings::default_settings;
use tuya_client::TuyaClient;

#[tokio::main]
async fn main() {
    let settings = default_settings();

    let tuya = TuyaClient::new(
        settings.headers.client_id.unwrap_or_default(),
        settings.secret.value.unwrap_or_default(),
        settings.base_url,
        settings.headers.sign_method,
        settings.token.access_token,
    );

    let device_ids: Vec<String> = settings
        .lights
        .values()
        .filter_map(|light| light.device_id.clone())
        .collect();

    tuya.request_token(&settings.token.expires_at).await;

    let online = tuya.online_devices(&device_ids).await;

    if online.is_empty() {
        println!("No devices online");
        return;
    }

    tuya.color(&online, "GREEN").await;
    tuya.state(&online, false).await;
    tuya.default(&online, &settings.default_settings).await;
}
