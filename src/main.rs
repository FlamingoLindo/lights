use crate::bulb::default::bulb_default;
use crate::bulb::state::bulb_state;
use crate::settings::settings::default_settings;
use crate::token::get_token::request_tuya_token;

mod bulb;
mod settings;
mod token;

#[tokio::main]
async fn main() {
    // Load Lights settings
    let settings = default_settings();

    // Create HTTP client
    let client = reqwest::Client::new();

    // Request auth token if needed
    request_tuya_token(
        &client,
        &settings.headers.client_id,
        &settings.secret.value,
        &settings.base_url,
        &settings.token.expires_at,
    )
    .await;

    // Get bulbs ids from settings file
    let device_ids: Vec<String> = settings
        .lights
        .values()
        .filter_map(|light| light.device_id.clone())
        .collect();

    // Control bulbs states (on/off)
    bulb_state(
        &settings.base_url,
        &client,
        &settings.headers.client_id,
        &settings.secret.value,
        &settings.token.access_token,
        &settings.headers.sign_method,
        &device_ids,
        false,
    )
    .await;

    bulb_default(
        &settings.base_url,
        &client,
        &settings.headers.client_id,
        &settings.secret.value,
        &device_ids,
        &settings.headers.sign_method,
        &settings.token.access_token,
        &settings.default_settings,
    )
    .await;
}
