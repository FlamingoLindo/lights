use twitch_eventsub::{
    EventSubError, Subscription, TwitchEventSubApi, TwitchEventSubApiBuilder, TwitchKeys,
};

pub fn create_twitch_builder(keys: TwitchKeys) -> TwitchEventSubApiBuilder {
    let builder = TwitchEventSubApi::builder(keys)
        .set_redirect_url("https://twitch-auth.flamingo-lindo.com.br/")
        .generate_new_token_if_none(true)
        .generate_access_token_on_expire(true)
        .auto_save_load_created_tokens(".user_token.env", ".refresh_token.env")
        .add_subscription(Subscription::ChannelFollow)
        .add_subscriptions(vec![
            Subscription::ChannelPointsCustomRewardRedeem,
            Subscription::ChannelPointsAutoRewardRedeem,
        ]);

    return builder;
}

pub fn build_twitch_connection(builder: TwitchEventSubApiBuilder) -> TwitchEventSubApi {
    let api = {
        match builder.build() {
            Ok(api) => {
                println!("Twitch connection established!");
                api
            }
            Err(EventSubError::TokenMissingScope) => {
                panic!(
                    "Reauthorisation of token is required for the token to have all the requested subscriptions."
                );
            }
            Err(EventSubError::NoSubscriptionsRequested) => {
                panic!("No subscriptions passed into builder!");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    };

    return api;
}
