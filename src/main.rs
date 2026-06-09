mod bulb;
mod clients;
mod settings;
mod token;
mod twitch;

use crate::{
    clients::{tuya_client::TuyaClient, twitch_client::TwitchClient},
    settings::settings::default_settings,
    token::get_token::request_tuya_token,
};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let settings = default_settings();
    let mut twitch = TwitchClient::new(&settings);

    let (initial_token, expires_in_secs) = request_tuya_token(
        &reqwest::Client::new(),
        &Some(settings.headers.client_id.clone().unwrap_or_default()),
        &Some(settings.secret.value.clone().unwrap_or_default()),
        &settings.base_url,
    )
    .await
    .expect("Failed to get initial Tuya token");

    let mut tuya = TuyaClient::new(
        settings.headers.client_id.unwrap_or_default(),
        settings.secret.value.unwrap_or_default(),
        settings.base_url,
        settings.headers.sign_method,
        Some(initial_token),
        Instant::now() + Duration::from_secs(expires_in_secs),
    );

    let device_ids: Vec<String> = settings
        .lights
        .values()
        .filter_map(|light| light.device_id.clone())
        .collect();

    let online = tuya.online_devices(&device_ids).await;

    loop {
        twitch
            .receive(&mut tuya, &online, &settings.default_settings)
            .await;
    }
}
