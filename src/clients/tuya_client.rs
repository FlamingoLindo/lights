use crate::bulb::bosnia::bosnian_bulbs;
use crate::bulb::color::bulb_color;
use crate::bulb::default::bulb_default;
use crate::bulb::offline::online_devices;
use crate::bulb::state::bulb_state;
use crate::settings::settings::DefaultSettings;
use crate::token::get_token::request_tuya_token;
use std::time::{Duration, Instant};

pub struct TuyaClient {
    http: reqwest::Client,
    client_id: String,
    secret: String,
    base_url: String,
    sign_method: String,
    access_token: Option<String>,
    token_expires_at: Instant,
}

impl TuyaClient {
    pub fn new(
        client_id: String,
        secret: String,
        base_url: String,
        sign_method: String,
        access_token: Option<String>,
        token_expires_at: Instant,
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            client_id,
            secret,
            base_url,
            sign_method,
            access_token,
            token_expires_at,
        }
    }

    async fn ensure_token_valid(&mut self) {
        let now = Instant::now();
        let already_expired = now >= self.token_expires_at;
        let expiring_soon =
            self.token_expires_at.saturating_duration_since(now) < Duration::from_secs(60);

        if already_expired || expiring_soon {
            println!(
                "Token {}. Refreshing...",
                if already_expired {
                    "expired"
                } else {
                    "expiring soon"
                }
            );
            if let Some((new_token, expires_in_secs)) = request_tuya_token(
                &self.http,
                &Some(self.client_id.clone()),
                &Some(self.secret.clone()),
                &self.base_url,
            )
            .await
            {
                self.access_token = Some(new_token);
                self.token_expires_at = Instant::now() + Duration::from_secs(expires_in_secs);
            } else {
                eprintln!("Failed to refresh token, requests may fail.");
            }
        }
    }

    pub async fn online_devices(&mut self, device_ids: &[String]) -> Vec<String> {
        self.ensure_token_valid().await;
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

    pub async fn color(&mut self, online: &[String], color: &str) {
        self.ensure_token_valid().await;
        bulb_color(
            &self.base_url,
            &self.http,
            &Some(self.client_id.clone()),
            &Some(self.secret.clone()),
            online,
            &self.sign_method,
            &self.access_token,
            color,
        )
        .await;
    }

    pub async fn state(&mut self, device_ids: &[String], on: bool) {
        self.ensure_token_valid().await;
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

    pub async fn default(&mut self, device_ids: &[String], default_settings: &DefaultSettings) {
        self.ensure_token_valid().await;
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

    pub async fn bosnia(&mut self, online: &[String]) {
        self.ensure_token_valid().await;
        bosnian_bulbs(
            &self.base_url,
            &self.http,
            &Some(self.client_id.clone()),
            &Some(self.secret.clone()),
            online,
            &self.sign_method,
            &self.access_token,
        )
        .await;
    }
}
