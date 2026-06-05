use twitch_eventsub::{
    EventSubError, Subscription, TwitchEventSubApi, TwitchEventSubApiBuilder, TwitchKeys,
};

/// Creates a pre-configured [`TwitchEventSubApiBuilder`] with the given keys.
///
/// Sets up the redirect URL, token management, and the following subscriptions:
/// - [`Subscription::ChannelFollow`]
/// - [`Subscription::ChannelPointsCustomRewardRedeem`]
/// - [`Subscription::ChannelPointsAutoRewardRedeem`]
///
/// Tokens are automatically saved to and loaded from `.user_token.env` and
/// `.refresh_token.env`. A new token is generated if none exists, and refreshed
/// automatically on expiry.
///
/// # Arguments
///
/// * `keys` - Twitch API credentials used to authenticate the connection
///
/// # Returns
///
/// A [`TwitchEventSubApiBuilder`] ready to be passed to [`build_twitch_connection`].
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

/// Consumes a [`TwitchEventSubApiBuilder`] and establishes the Twitch EventSub connection.
///
/// # Arguments
///
/// * `builder` - A configured builder, typically created via [`create_twitch_builder`]
///
/// # Returns
///
/// A connected [`TwitchEventSubApi`] instance ready to receive events.
///
/// # Panics
///
/// Panics on any of the following conditions:
/// - [`EventSubError::TokenMissingScope`] — the token lacks required scopes and
///   must be reauthorised manually
/// - [`EventSubError::NoSubscriptionsRequested`] — the builder had no subscriptions
/// - Any other [`EventSubError`] variant — printed via `{:?}` before panicking
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
