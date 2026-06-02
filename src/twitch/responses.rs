use std::time::Duration;

use twitch_eventsub::{Event, ResponseType, TwitchEventSubApi};

pub fn get_responses(api: &mut TwitchEventSubApi) {
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
                }
                _ => {}
            },
            ResponseType::Close => println!("Twitch requested socket close"),
            _ => {}
        }
    }
}
