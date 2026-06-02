use std::fs;

use twitch_eventsub::{TokenAccess, TwitchKeys};

use crate::settings::{self, settings::Settings};

pub fn get_twitch_keys(settings: &Settings) -> TwitchKeys {
    let twitch_keys = TwitchKeys {
        client_id: settings.twitch.client_id.clone().unwrap_or_default(),
        client_secret: settings.twitch.client_secret.clone().unwrap_or_default(),
        broadcaster_account_id: settings.twitch.broadcaster_id.clone().unwrap_or_default(),
        access_token: settings
            .twitch
            .access_token
            .clone()
            .map(|t| TokenAccess::User(t)),
        refresh_token: settings.twitch.refresh_token.clone(),
        authorisation_code: None,
        sender_account_id: None,
    };
    twitch_keys
}

pub fn twitch_tokens_to_settings() {
    let access = fs::read_to_string(".user_token.env").unwrap_or_default();
    let refresh = fs::read_to_string(".refresh_token.env").unwrap_or_default();
    if !access.trim().is_empty() {
        settings::settings::save_twitch_tokens(
            access.trim().to_string(),
            refresh.trim().to_string(),
        );
    }
}
