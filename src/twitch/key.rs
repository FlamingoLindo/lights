use std::fs;

use twitch_eventsub::{TokenAccess, TwitchKeys};

use crate::settings::{self, settings::Settings};

/// Constructs a [`TwitchKeys`] instance from the application settings.
///
/// Maps Twitch credentials from [`Settings`] into the [`TwitchKeys`] struct
/// expected by the Twitch EventSub API. Missing optional fields default to
/// empty strings. `authorisation_code` and `sender_account_id` are always
/// set to `None`.
///
/// # Arguments
///
/// * `settings` - Application settings containing Twitch credentials
///
/// # Returns
///
/// A [`TwitchKeys`] instance ready to be passed to [`create_twitch_builder`].
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

/// Reads saved Twitch tokens from disk and persists them to the application settings.
///
/// Reads `.user_token.env` and `.refresh_token.env`, and if the access token
/// is non-empty, calls [`save_twitch_tokens`] to store both values. This is
/// intended to be called on startup to restore tokens saved by the Twitch
/// OAuth flow.
///
/// If either file is missing or unreadable, the read silently defaults to an
/// empty string. If the access token is empty after trimming, nothing is saved.
///
/// # Errors
///
/// Does not return errors — file read failures fall back to empty strings
/// via [`unwrap_or_default`].
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
