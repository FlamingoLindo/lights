use chrono::Local;
use hmac::{Hmac, Mac};
use palette::Hsv;
use palette::IntoColor;
use palette::Srgb;
use palette::named;
use phf::phf_map;
use sha2::{Digest, Sha256};

use crate::bulb::common::CommandValue;
use crate::bulb::common::{Body, Command, Response};
use crate::settings::settings::{save_sign, save_time_stamp};
type HmacSha256 = Hmac<Sha256>;

static COLORS: phf::Map<&'static str, Srgb<u8>> = phf_map! {
    "ALICEBLUE" => named::ALICEBLUE,
    "ANTIQUEWHITE" => named::ANTIQUEWHITE,
    "AQUA" => named::AQUA,
    "AQUAMARINE" => named::AQUAMARINE,
    "AZURE" => named::AZURE,
    "BEIGE" => named::BEIGE,
    "BISQUE" => named::BISQUE,
    "BLACK" => named::BLACK,
    "BLANCHEDALMOND" => named::BLANCHEDALMOND,
    "BLUE" => named::BLUE,
    "BLUEVIOLET" => named::BLUEVIOLET,
    "BROWN" => named::BROWN,
    "BURLYWOOD" => named::BURLYWOOD,
    "CADETBLUE" => named::CADETBLUE,
    "CHARTREUSE" => named::CHARTREUSE,
    "CHOCOLATE" => named::CHOCOLATE,
    "CORAL" => named::CORAL,
    "CORNFLOWERBLUE" => named::CORNFLOWERBLUE,
    "CORNSILK" => named::CORNSILK,
    "CRIMSON" => named::CRIMSON,
    "CYAN" => named::CYAN,
    "DARKBLUE" => named::DARKBLUE,
    "DARKCYAN" => named::DARKCYAN,
    "DARKGOLDENROD" => named::DARKGOLDENROD,
    "DARKGRAY" => named::DARKGRAY,
    "DARKGREEN" => named::DARKGREEN,
    "DARKGREY" => named::DARKGREY,
    "DARKKHAKI" => named::DARKKHAKI,
    "DARKMAGENTA" => named::DARKMAGENTA,
    "DARKOLIVEGREEN" => named::DARKOLIVEGREEN,
    "DARKORANGE" => named::DARKORANGE,
    "DARKORCHID" => named::DARKORCHID,
    "DARKRED" => named::DARKRED,
    "DARKSALMON" => named::DARKSALMON,
    "DARKSEAGREEN" => named::DARKSEAGREEN,
    "DARKSLATEBLUE" => named::DARKSLATEBLUE,
    "DARKSLATEGRAY" => named::DARKSLATEGRAY,
    "DARKSLATEGREY" => named::DARKSLATEGREY,
    "DARKTURQUOISE" => named::DARKTURQUOISE,
    "DARKVIOLET" => named::DARKVIOLET,
    "DEEPPINK" => named::DEEPPINK,
    "DEEPSKYBLUE" => named::DEEPSKYBLUE,
    "DIMGRAY" => named::DIMGRAY,
    "DIMGREY" => named::DIMGREY,
    "DODGERBLUE" => named::DODGERBLUE,
    "FIREBRICK" => named::FIREBRICK,
    "FLORALWHITE" => named::FLORALWHITE,
    "FORESTGREEN" => named::FORESTGREEN,
    "FUCHSIA" => named::FUCHSIA,
    "GAINSBORO" => named::GAINSBORO,
    "GHOSTWHITE" => named::GHOSTWHITE,
    "GOLD" => named::GOLD,
    "GOLDENROD" => named::GOLDENROD,
    "GRAY" => named::GRAY,
    "GREEN" => named::GREEN,
    "GREENYELLOW" => named::GREENYELLOW,
    "GREY" => named::GREY,
    "HONEYDEW" => named::HONEYDEW,
    "HOTPINK" => named::HOTPINK,
    "INDIANRED" => named::INDIANRED,
    "INDIGO" => named::INDIGO,
    "IVORY" => named::IVORY,
    "KHAKI" => named::KHAKI,
    "LAVENDER" => named::LAVENDER,
    "LAVENDERBLUSH" => named::LAVENDERBLUSH,
    "LAWNGREEN" => named::LAWNGREEN,
    "LEMONCHIFFON" => named::LEMONCHIFFON,
    "LIGHTBLUE" => named::LIGHTBLUE,
    "LIGHTCORAL" => named::LIGHTCORAL,
    "LIGHTCYAN" => named::LIGHTCYAN,
    "LIGHTGOLDENRODYELLOW" => named::LIGHTGOLDENRODYELLOW,
    "LIGHTGRAY" => named::LIGHTGRAY,
    "LIGHTGREEN" => named::LIGHTGREEN,
    "LIGHTGREY" => named::LIGHTGREY,
    "LIGHTPINK" => named::LIGHTPINK,
    "LIGHTSALMON" => named::LIGHTSALMON,
    "LIGHTSEAGREEN" => named::LIGHTSEAGREEN,
    "LIGHTSKYBLUE" => named::LIGHTSKYBLUE,
    "LIGHTSLATEGRAY" => named::LIGHTSLATEGRAY,
    "LIGHTSLATEGREY" => named::LIGHTSLATEGREY,
    "LIGHTSTEELBLUE" => named::LIGHTSTEELBLUE,
    "LIGHTYELLOW" => named::LIGHTYELLOW,
    "LIME" => named::LIME,
    "LIMEGREEN" => named::LIMEGREEN,
    "LINEN" => named::LINEN,
    "MAGENTA" => named::MAGENTA,
    "MAROON" => named::MAROON,
    "MEDIUMAQUAMARINE" => named::MEDIUMAQUAMARINE,
    "MEDIUMBLUE" => named::MEDIUMBLUE,
    "MEDIUMORCHID" => named::MEDIUMORCHID,
    "MEDIUMPURPLE" => named::MEDIUMPURPLE,
    "MEDIUMSEAGREEN" => named::MEDIUMSEAGREEN,
    "MEDIUMSLATEBLUE" => named::MEDIUMSLATEBLUE,
    "MEDIUMSPRINGGREEN" => named::MEDIUMSPRINGGREEN,
    "MEDIUMTURQUOISE" => named::MEDIUMTURQUOISE,
    "MEDIUMVIOLETRED" => named::MEDIUMVIOLETRED,
    "MIDNIGHTBLUE" => named::MIDNIGHTBLUE,
    "MINTCREAM" => named::MINTCREAM,
    "MISTYROSE" => named::MISTYROSE,
    "MOCCASIN" => named::MOCCASIN,
    "NAVAJOWHITE" => named::NAVAJOWHITE,
    "NAVY" => named::NAVY,
    "OLDLACE" => named::OLDLACE,
    "OLIVE" => named::OLIVE,
    "OLIVEDRAB" => named::OLIVEDRAB,
    "ORANGE" => named::ORANGE,
    "ORANGERED" => named::ORANGERED,
    "ORCHID" => named::ORCHID,
    "PALEGOLDENROD" => named::PALEGOLDENROD,
    "PALEGREEN" => named::PALEGREEN,
    "PALETURQUOISE" => named::PALETURQUOISE,
    "PALEVIOLETRED" => named::PALEVIOLETRED,
    "PAPAYAWHIP" => named::PAPAYAWHIP,
    "PEACHPUFF" => named::PEACHPUFF,
    "PERU" => named::PERU,
    "PINK" => named::PINK,
    "PLUM" => named::PLUM,
    "POWDERBLUE" => named::POWDERBLUE,
    "PURPLE" => named::PURPLE,
    "REBECCAPURPLE" => named::REBECCAPURPLE,
    "RED" => named::RED,
    "ROSYBROWN" => named::ROSYBROWN,
    "ROYALBLUE" => named::ROYALBLUE,
    "SADDLEBROWN" => named::SADDLEBROWN,
    "SALMON" => named::SALMON,
    "SANDYBROWN" => named::SANDYBROWN,
    "SEAGREEN" => named::SEAGREEN,
    "SEASHELL" => named::SEASHELL,
    "SIENNA" => named::SIENNA,
    "SILVER" => named::SILVER,
    "SKYBLUE" => named::SKYBLUE,
    "SLATEBLUE" => named::SLATEBLUE,
    "SLATEGRAY" => named::SLATEGRAY,
    "SLATEGREY" => named::SLATEGREY,
    "SNOW" => named::SNOW,
    "SPRINGGREEN" => named::SPRINGGREEN,
    "STEELBLUE" => named::STEELBLUE,
    "TAN" => named::TAN,
    "TEAL" => named::TEAL,
    "THISTLE" => named::THISTLE,
    "TOMATO" => named::TOMATO,
    "TURQUOISE" => named::TURQUOISE,
    "VIOLET" => named::VIOLET,
    "WHEAT" => named::WHEAT,
    "WHITE" => named::WHITE,
    "WHITESMOKE" => named::WHITESMOKE,
    "YELLOW" => named::YELLOW,
    "YELLOWGREEN" => named::YELLOWGREEN,
};

fn color_from_str(name: &str) -> Option<Srgb<u8>> {
    COLORS.get(name.to_uppercase().as_str()).copied()
}

/// Colors: docs.rs/palette/latest/palette/named/index.html
pub async fn bulb_color(
    base_url: &String,
    client: &reqwest::Client,
    client_id: &Option<String>,
    secret: &Option<String>,
    devices_ids: &Vec<String>,
    sign_method: &String,
    access_token: &Option<String>,
    color_name: &str,
) {
    let color = match color_from_str(color_name) {
        Some(c) => c,
        None => {
            eprintln!("Unknown color: {}", color_name);
            return;
        }
    };

    let client_id_value = client_id.as_deref().unwrap_or("");
    let secret_value = secret.as_deref().unwrap_or("");
    let access_token_value = access_token.as_deref().unwrap_or("");

    for (i, id) in devices_ids.iter().enumerate() {
        let t = Local::now().timestamp_millis().to_string();
        save_time_stamp(t.clone());

        let srgb = Srgb::<f32>::from_format(color);

        let hsv: Hsv = srgb.into_color();

        let h = hsv.hue.into_degrees().round() as i32;
        let s = (hsv.saturation * 1000.0).round() as i32;
        let v = (hsv.value * 1000.0).round() as i32;

        let body = Body {
            commands: vec![
                Command {
                    code: "work_mode".to_string(),
                    value: CommandValue::Str("colour".to_string()),
                },
                Command {
                    code: "colour_data_v2".to_string(),
                    value: CommandValue::Map(serde_json::json!({
                        "h": h,
                        "s": s,
                        "v": v
                    })),
                },
            ],
        };

        let body_json = serde_json::to_string(&body).expect("Failed to serialize body");

        let body_hash = format!("{:x}", Sha256::digest(body_json.as_bytes()));

        let path = format!("/v1.0/devices/{id}/commands");

        let content_to_sign = format!("POST\n{}\n\n{}", body_hash, path);

        let string_to_sign = format!(
            "{}{}{}{}",
            client_id_value, access_token_value, t, content_to_sign
        );

        let mut mac =
            HmacSha256::new_from_slice(secret_value.as_bytes()).expect("Invalid HMAC key");

        mac.update(string_to_sign.as_bytes());

        let sign = hex::encode(mac.finalize().into_bytes()).to_uppercase();
        save_sign(&sign);

        let url = format!("{base_url}{path}");

        match client
            .post(url)
            .header("client_id", client_id_value)
            .header("access_token", access_token_value)
            .header("t", &t)
            .header("sign", &sign)
            .header("sign_method", sign_method)
            .body(body_json)
            .send()
            .await
        {
            Ok(response) => match response.json::<Response>().await {
                Ok(_) => {
                    println!("Bulb {i} color changed")
                }
                Err(err) => {
                    eprintln!("Failed to parse response: {err}");
                }
            },
            Err(err) => {
                eprintln!("Request failed: {err}");
            }
        }
    }
}
