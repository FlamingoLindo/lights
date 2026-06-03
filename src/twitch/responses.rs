use std::time::{Duration, Instant};

use twitch_eventsub::{Event, ResponseType, TwitchEventSubApi};

use crate::{clients::tuya_client::TuyaClient, settings::settings::DefaultSettings};

const RESTORE_AFTER_SECS: u64 = 5;

pub async fn get_responses(
    api: &mut TwitchEventSubApi,
    tuya: &TuyaClient,
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
                    if reward.reward.title == "Change Lights Color" {
                        tuya.color(online, &reward.user_input.to_string()).await;
                        *last_color_change = Some(Instant::now());
                    }
                    if reward.reward.title == "Turn Lights Off" {
                        tuya.state(online, false).await;
                        *last_color_change = Some(Instant::now());
                    }
                }
                _ => {}
            },
            ResponseType::Close => println!("Twitch requested socket close"),
            _ => {}
        }
    }

    if let Some(changed_at) = *last_color_change {
        if changed_at.elapsed() >= Duration::from_secs(RESTORE_AFTER_SECS) {
            println!("Restoring default light settings...");
            tuya.default(online, default_settings).await;
            *last_color_change = None;
        }
    }
}
