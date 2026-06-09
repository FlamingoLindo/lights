use std::time::Instant;

use twitch_eventsub::TwitchEventSubApi;

use crate::{
    clients::tuya_client::TuyaClient,
    settings::settings::{DefaultSettings, Settings},
    twitch::{
        builder::{build_twitch_connection, create_twitch_builder},
        key::{get_twitch_keys, twitch_tokens_to_settings},
        responses::get_responses,
    },
};

pub struct TwitchClient {
    api: TwitchEventSubApi,
    last_color_change: Option<Instant>,
}

impl TwitchClient {
    pub fn new(settings: &Settings) -> Self {
        let keys = get_twitch_keys(settings);
        let builder = create_twitch_builder(keys);
        let api = build_twitch_connection(builder);
        twitch_tokens_to_settings();
        Self {
            api,
            last_color_change: None,
        }
    }

    pub async fn receive(
        &mut self,
        tuya: &mut TuyaClient,
        online: &[String],
        default_settings: &DefaultSettings,
    ) {
        get_responses(
            &mut self.api,
            tuya,
            online,
            default_settings,
            &mut self.last_color_change,
        )
        .await;
    }
}
