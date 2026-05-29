use crate::settings::settings::default_settings;
use crate::token::get_token::request_tuya_token;

mod settings;
mod token;

#[tokio::main]
async fn main() {
    let settings = default_settings();

    request_tuya_token(
        &settings.headers.client_id,
        &settings.secret.value,
        &settings.base_url,
        &settings.token.expires_at,
    )
    .await;
}
