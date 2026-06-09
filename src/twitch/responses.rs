use std::time::{Duration, Instant};

use twitch_eventsub::{Event, ResponseType, TwitchEventSubApi};

use crate::{clients::tuya_client::TuyaClient, settings::settings::DefaultSettings};

// const RESTORE_AFTER_SECS: u64 = 5;

/// Processes all pending Twitch EventSub messages and drives light control accordingly.
///
/// Polls the Twitch API for messages with a 100ms timeout and handles the
/// following channel point redemptions:
/// - `"Change Lights Color"` — calls [`TuyaClient::color`] with the user's input
/// - `"Turn Lights Off"` — calls [`TuyaClient::state`] with `false`
///
/// Both redemptions update `last_color_change` to the current [`Instant`].
/// After [`RESTORE_AFTER_SECS`] seconds have elapsed since the last change,
/// [`TuyaClient::default`] is called automatically to restore the lights,
/// and `last_color_change` is reset to `None`.
///
/// # Arguments
///
/// * `api` - Mutable reference to the active Twitch EventSub connection
/// * `tuya` - Tuya client used to send commands to the smart bulbs
/// * `online` - Slice of online device IDs to target
/// * `default_settings` - Settings to restore after the timer expires
/// * `last_color_change` - Tracks when the last light-affecting redemption
///   occurred. Modified in place by this function.
///
/// # Errors
///
/// Does not return errors — unhandled [`ResponseType`] and [`Event`] variants
/// are silently ignored. A [`ResponseType::Close`] is logged to stdout but
/// not acted upon.
pub async fn get_responses(
    api: &mut TwitchEventSubApi,
    tuya: &mut TuyaClient,
    online: &[String],
    default_settings: &DefaultSettings,
    last_color_change: &mut Option<Instant>,
) {
    let responses = api.receive_all_messages(Some(Duration::from_millis(100)));
    for response in responses {
        match response {
            ResponseType::Event(event) => match event {
                Event::PointsCustomRewardRedeem(reward) => {
                    println!(
                        "{} redeemed {} with {} Channel Points: {}",
                        reward.user.name,
                        reward.reward.title,
                        reward.reward.cost,
                        reward.user_input,
                    );
                    // Custom color
                    if reward.reward.title == "Change Lights Color" {
                        tuya.color(online, &reward.user_input.to_string()).await;
                        *last_color_change = Some(Instant::now());
                    }
                    // Turn lights off
                    if reward.reward.title == "Turn Lights Off" {
                        tuya.state(online, false).await;
                        *last_color_change = Some(Instant::now());
                    }
                    // Turn lights on
                    if reward.reward.title == "Turn Lights On" {
                        tuya.state(online, true).await;
                        tuya.default(online, default_settings).await;
                        *last_color_change = Some(Instant::now());
                    }
                    // Bosnia lights
                    if reward.reward.title == "Bosnian Lights" {
                        tuya.state(online, true).await;
                        tuya.bosnia(online).await;
                        *last_color_change = Some(Instant::now());
                    }
                }
                _ => {}
            },
            ResponseType::Close => println!("Twitch requested socket close"),
            _ => {}
        }
    }

    // if let Some(changed_at) = *last_color_change {
    //     if changed_at.elapsed() >= Duration::from_secs(RESTORE_AFTER_SECS) {
    //         println!("Restoring default light settings...");
    //         tuya.default(online, default_settings).await;
    //         *last_color_change = None;
    //     }
    // }
}
