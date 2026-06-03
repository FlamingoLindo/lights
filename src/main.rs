mod bulb;
mod clients;
mod settings;
mod token;
mod twitch;

use crate::{
    clients::{tuya_client::TuyaClient, twitch_client::TwitchClient},
    settings::settings::default_settings,
};

#[tokio::main]
async fn main() {
    let settings = default_settings();

    let mut twitch = TwitchClient::new(&settings);

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

    loop {
        twitch.receive(&tuya, &online, &settings.default_settings).await;
    }
}
