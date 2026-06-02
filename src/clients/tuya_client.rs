use crate::bulb::color::bulb_color;
use crate::bulb::default::bulb_default;
use crate::bulb::offline::online_devices;
use crate::bulb::state::bulb_state;
use crate::settings::settings::DefaultSettings;
use crate::token::get_token::request_tuya_token;

pub struct TuyaClient {
    http: reqwest::Client,
    client_id: String,
    secret: String,
    base_url: String,
    sign_method: String,
    access_token: Option<String>,
}

impl TuyaClient {
    pub fn new(
        client_id: String,
        secret: String,
        base_url: String,
        sign_method: String,
        access_token: Option<String>,
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            client_id,
            secret,
            base_url,
            sign_method,
            access_token,
        }
    }

    pub async fn online_devices(&self, device_ids: &Vec<String>) -> Vec<String> {
        online_devices(
            &self.base_url,
            &self.http,
            &Some(self.client_id.clone()),
            &Some(self.secret.clone()),
            &self.access_token,
            &self.sign_method,
            device_ids,
        )
        .await
    }

    pub async fn request_token(&self, expires_at: &Option<i64>) {
        request_tuya_token(
            &self.http,
            &Some(self.client_id.clone()),
            &Some(self.secret.clone()),
            &self.base_url,
            expires_at,
        )
        .await;
    }

    pub async fn color(&self, device_ids: &Vec<String>, color_name: &str) {
        bulb_color(
            &self.base_url,
            &self.http,
            &Some(self.client_id.clone()),
            &Some(self.secret.clone()),
            device_ids,
            &self.sign_method,
            &self.access_token,
            color_name,
        )
        .await;
    }

    pub async fn state(&self, device_ids: &Vec<String>, on: bool) {
        bulb_state(
            &self.base_url,
            &self.http,
            &Some(self.client_id.clone()),
            &Some(self.secret.clone()),
            &self.access_token,
            &self.sign_method,
            device_ids,
            on,
        )
        .await;
    }

    pub async fn default(&self, device_ids: &Vec<String>, default_settings: &DefaultSettings) {
        bulb_default(
            &self.base_url,
            &self.http,
            &Some(self.client_id.clone()),
            &Some(self.secret.clone()),
            device_ids,
            &self.sign_method,
            &self.access_token,
            default_settings,
        )
        .await;
    }
}
