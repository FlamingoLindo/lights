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

    for (name, light) in settings.lights.iter_mut() {
        if light.device_id.as_deref().unwrap_or("").is_empty() {
            light.device_id = Some(prompt(&format!(
                "No device_id set for '{}', please enter it:",
                name
            )));
        }
    }
}

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
        }
    };

    fill_missing(&mut settings);

    let toml_string = toml::to_string(&settings).expect("Failed to serialize settings");
    fs::write(path, toml_string).expect("Failed to write settings.toml");

    settings
}

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
