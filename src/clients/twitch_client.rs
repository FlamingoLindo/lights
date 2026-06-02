use twitch_eventsub::TwitchEventSubApi;

use crate::{settings::settings::Settings, twitch::{builder::{build_twitch_connection, create_twitch_builder}, key::{get_twitch_keys, twitch_tokens_to_settings}, responses::get_responses}};

pub struct TwitchClient {
    api: TwitchEventSubApi,
}

impl TwitchClient {
    pub fn new(settings: &Settings) -> Self {
        let keys = get_twitch_keys(settings);
        let builder = create_twitch_builder(keys);
        let api = build_twitch_connection(builder);
        twitch_tokens_to_settings();
        Self { api }
    }

    pub fn receive(&mut self) {
        get_responses(&mut self.api);
    }
}
