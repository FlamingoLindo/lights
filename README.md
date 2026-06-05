# Lights project

## **What this project does**

- Integrates Twitch chat with Tuya smart bulbs so viewers can change light colors and power state.
- Uses the Tuya Cloud API for device control and OAuth token management.

<https://github.com/user-attachments/assets/6d80d77f-7e90-4176-96ff-7d9d2db4b1a1>

### **How it works (high level)**

- `src/main.rs`: bootstraps the application, creates the `TwitchClient` and `TuyaClient`, and enters the main receive loop.

```rust
mod bulb;
mod clients;
mod settings;
mod token;
mod twitch;

#[tokio::main]
async fn main() {
    let settings = default_settings();

    let mut twitch = TwitchClient::new(&settings);

    let tuya = TuyaClient::new(/* client id, secret, base url, ... */);
    // ... request token, check online devices, and loop receiving events
}
```

- `src/clients/twitch_client.rs`: connects to Twitch EventSub, translates chat events into responses, and delegates bulb actions to the Tuya client.

```rust
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
        Self { api, last_color_change: None }
    }
}
```

- `src/bulb/color.rs`: converts CSS color names to HSV and sends signed `colour_data_v2` commands to Tuya devices (HMAC-SHA256 signing per request).

```rust
pub async fn bulb_color(
    base_url: &String,
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    devices_ids: &[String],
    sign_method: &String,
    access_token: &Option<String>,
    color_name: &str,
) {
    // lookup color, convert to HSV, build command body, sign, and POST to Tuya
}
```

These snippets are small excerpts; see the real implementations in [src/main.rs](src/main.rs), [src/clients/twitch_client.rs](src/clients/twitch_client.rs), and [src/bulb/color.rs](src/bulb/color.rs).

## How to get Tuya keys

1. Head over to [Tuya Developer Platform](https://platform.tuya.com/)
2. Go to the `Cloud` tab and click on `Create Cloud Project`
    ![image](https://github.com/user-attachments/assets/01b36815-9dba-45fa-8237-40fc6c34f8e0)
3. Fill in with the following information:
    - Project name: your choice;
    - Description: optional;
    - Industry: select `Smart Home`;
    - Development Method: select `Smart Home`;
    - Data center: this you have to select the one according to your smart device, in my case its `Western America Data Center`;
    ![image2](https://github.com/user-attachments/assets/5699272b-d1be-447b-b9a4-b645c1893482)
4. Select this options:
    - IoT Core
    - Authorization Token Management
    - Smart home Basic Service
    - Data Dashboard Service
    - [Deprecate] Smart Home Scene Linkage
    - Product Management Service
    - Device Status Notification
    - Smart Home Content Manage
    ![image3](https://github.com/user-attachments/assets/113d4358-95ff-4c8a-9919-d998f31f7bc4)
5. In the `Overview` copy your `Access ID/Client ID` and `Access Secret/Client Secret`.
6. Head over to the `Devices` tab, and click on `Add Device`
    - In my case I went with the `Add Device with Tuya Spatial App` option
    ![image4](https://github.com/user-attachments/assets/8f00f2f7-85b8-44db-9892-599a38c05c98)
7. Copy the `id` of each device (light bulb)
    ![image5](https://github.com/user-attachments/assets/fdce0358-ab9b-4dcb-b391-fbdf6e72022f)

## How to get Twitch uya keys

1. Head on over to [Twitch Dev Platform](https://dev.twitch.tv/console/apps)
2. Click on the `+ Register your application` button:
    ![image6](https://github.com/user-attachments/assets/e7ed1e22-7a8f-4e0c-b6a7-e3eb9509797c)
3. Fill out the form with something similar to this:
    ![image7](https://github.com/user-attachments/assets/05fc8dc2-8750-44df-98d1-e58ec86dfe55)
4. Then click click on the `Manage` button
5. Here you will need to copy your `Client ID` and request a new `Client Secret`:
    ![image8](https://github.com/user-attachments/assets/2e640a74-8428-492b-a4d9-22154fa4cbe4)
