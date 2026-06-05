use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub base_url: String,
    pub token: Token,
    pub secret: Secret,
    pub headers: Headers,
    pub lights: HashMap<String, LightSettings>,
    pub default_settings: DefaultSettings,
    pub twitch: TwitchSettings,
}

#[derive(Serialize, Deserialize)]
/// https://dev.twitch.tv/console/apps
pub struct TwitchSettings {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub broadcaster_id: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Secret {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Headers {
    pub client_id: Option<String>,

    pub sign_method: String,
    pub t: Option<String>,
    pub sign: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LightSettings {
    pub device_id: Option<String>,
    pub switch_led: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultSettings {
    pub work_mode: String,
    pub bright_value_v2: i32,
    pub temp_value_v2: i32,
}

impl Default for LightSettings {
    fn default() -> Self {
        Self {
            device_id: None,
            switch_led: true,
        }
    }
}

fn prompt(label: &str) -> String {
    println!("{}", label);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn fill_missing(settings: &mut Settings) {
    if settings.secret.value.as_deref().unwrap_or("").is_empty() {
        settings.secret.value = Some(prompt("No secret set, please enter it:"));
    }

    if settings
        .headers
        .client_id
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        settings.headers.client_id = Some(prompt("No client_id set, please enter it:"));
    }

    if settings
        .twitch
        .client_id
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        settings.twitch.client_id = Some(prompt("No twitch client_id set, please enter it"))
    }

    if settings
        .twitch
        .client_secret
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        settings.twitch.client_secret = Some(prompt("No twitch client_secret set, please enter it"))
    }

    if settings
        .twitch
        .broadcaster_id
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        settings.twitch.broadcaster_id =
            Some(prompt("No twitch broadcaster_id set, please enter it"))
    }

    for (name, light) in settings.lights.iter_mut() {
        if light.device_id.as_deref().unwrap_or("").is_empty() {
            light.device_id = Some(prompt(&format!(
                "No device_id set for '{}', please enter it:",
                name
            )));
        }
    }
}

/// Loads settings from `settings.toml`, prompts for any missing required
/// fields, and writes the completed settings back to disk.
///
/// If `settings.toml` does not exist, a new file is created with default
/// values before prompting. The following fields trigger an interactive
/// prompt if empty:
/// - `secret.value`
/// - `headers.client_id`
/// - `twitch.client_id`, `twitch.client_secret`, `twitch.broadcaster_id`
/// - `device_id` for each entry in `lights`
///
/// # Returns
///
/// The fully populated [`Settings`] instance.
///
/// # Panics
///
/// Panics if `settings.toml` cannot be read, parsed, or written.
pub fn default_settings() -> Settings {
    let path = Path::new("settings.toml");

    let mut settings = if path.exists() {
        let contents = fs::read_to_string(path).expect("Failed to read settings.toml");
        toml::from_str(&contents).expect("Failed to parse settings.toml")
    } else {
        Settings {
            base_url: "https://openapi.tuyaus.com".to_string(),
            token: Token {
                access_token: Some("".to_string()),
                refresh_token: Some("".to_string()),
                expires_at: Some(0),
            },
            secret: Secret {
                value: Some("".to_string()),
            },
            headers: Headers {
                client_id: Some("".to_string()),
                sign_method: "HMAC-SHA256".to_string(),
                t: Some("".to_string()),
                sign: Some("".to_string()),
            },
            lights: {
                let mut map = HashMap::new();
                map.insert(
                    "source1".to_string(),
                    LightSettings {
                        device_id: Some("".to_string()),
                        ..LightSettings::default()
                    },
                );
                map.insert(
                    "source2".to_string(),
                    LightSettings {
                        device_id: Some("".to_string()),
                        ..LightSettings::default()
                    },
                );
                map
            },
            default_settings: DefaultSettings {
                work_mode: "white".to_string(),
                bright_value_v2: 480,
                temp_value_v2: 1000,
            },
            twitch: TwitchSettings {
                client_id: Some("".to_string()),
                client_secret: Some("".to_string()),
                broadcaster_id: Some("".to_string()),
                access_token: Some("".to_string()),
                refresh_token: Some("".to_string()),
            },
        }
    };

    fill_missing(&mut settings);

    let toml_string = toml::to_string(&settings).expect("Failed to serialize settings");
    fs::write(path, toml_string).expect("Failed to write settings.toml");

    settings
}

/// Updates `headers.t` in `settings.toml` with the given timestamp.
///
/// # Panics
///
/// Panics if `settings.toml` is missing, cannot be parsed, or cannot be written.
pub fn save_time_stamp(time_stamp: String) {
    let path = Path::new("settings.toml");

    let mut settings: Settings = if path.exists() {
        let contents = fs::read_to_string(path).expect("Failed to read settings.toml");
        toml::from_str(&contents).expect("Failed to parse settings.toml")
    } else {
        panic!("settings.toml not found");
    };

    settings.headers.t = Some(time_stamp);

    let toml_string = toml::to_string(&settings).expect("Failed to serialize settings");
    fs::write(path, toml_string).expect("Failed to write settings.toml");
}

/// Updates `headers.sign` in `settings.toml` with the given signature.
///
/// # Panics
///
/// Panics if `settings.toml` is missing, cannot be parsed, or cannot be written.
pub fn save_sign(sign: &str) {
    let path = Path::new("settings.toml");

    let mut settings: Settings = if path.exists() {
        let contents = fs::read_to_string(path).expect("Failed to read settings.toml");
        toml::from_str(&contents).expect("Failed to parse settings.toml")
    } else {
        panic!("settings.toml not found");
    };

    settings.headers.sign = Some(sign.to_string());

    let toml_string = toml::to_string(&settings).expect("Failed to serialize settings");
    fs::write(path, toml_string).expect("Failed to write settings.toml");
}

/// Persists a new Tuya access token, refresh token, and expiry to `settings.toml`.
///
/// `expire_time` is a duration in seconds from the current time; the absolute
/// expiry timestamp is computed as `now + expire_time` and stored in
/// `token.expires_at`.
///
/// # Panics
///
/// Panics if `settings.toml` is missing, cannot be parsed, or cannot be written.
pub fn save_token(access_token: String, refresh_token: String, expire_time: i64) {
    let path = Path::new("settings.toml");

    let mut settings: Settings = if path.exists() {
        let contents = fs::read_to_string(path).expect("Failed to read settings.toml");
        toml::from_str(&contents).expect("Failed to parse settings.toml")
    } else {
        panic!("settings.toml not found");
    };

    settings.token.access_token = Some(access_token.to_string());
    settings.token.refresh_token = Some(refresh_token.to_string());

    let expires_at = Local::now().timestamp() + expire_time;
    settings.token.expires_at = Some(expires_at);

    let toml_string = toml::to_string(&settings).expect("Failed to serialize settings");
    fs::write(path, toml_string).expect("Failed to write settings.toml");
}

/// Updates `switch_led` for all entries in `lights` in `settings.toml`.
///
/// # Panics
///
/// Panics if `settings.toml` is missing, cannot be parsed, or cannot be written.
pub fn save_led_state(state: bool) {
    let path = Path::new("settings.toml");

    let mut settings: Settings = if path.exists() {
        let contents = fs::read_to_string(path).expect("Failed to read settings.toml");
        toml::from_str(&contents).expect("Failed to parse settings.toml")
    } else {
        panic!("settings.toml not found");
    };

    for light in settings.lights.values_mut() {
        light.switch_led = state;
    }

    let toml_string = toml::to_string(&settings).expect("Failed to serialize settings");

    fs::write(path, toml_string).expect("Failed to write settings.toml");
}

/// Persists new Twitch access and refresh tokens to `settings.toml`.
///
/// # Panics
///
/// Panics if `settings.toml` is missing, cannot be parsed, or cannot be written.
pub fn save_twitch_tokens(access_token: String, refresh_token: String) {
    let path = Path::new("settings.toml");

    let mut settings: Settings = if path.exists() {
        let contents = fs::read_to_string(path).expect("Failed to read settings.toml");
        toml::from_str(&contents).expect("Failed to parse settings.toml")
    } else {
        panic!("settings.toml not found");
    };

    settings.twitch.access_token = Some(access_token);
    settings.twitch.refresh_token = Some(refresh_token);

    let toml_string = toml::to_string(&settings).expect("Failed to serialize settings");
    fs::write(path, toml_string).expect("Failed to write settings.toml");
}
