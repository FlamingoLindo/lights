// use palette::Hsv;
// use palette::IntoColor;
// use palette::Srgb;
// use palette::named;

// docs.rs/palette/latest/palette/named/index.html

// let srgb = Srgb::<f32>::from_format(named::GREEN);

// let hsv: Hsv = srgb.into_color();

// let h = hsv.hue.into_degrees().round() as i32;
// let s = (hsv.saturation * 1000.0).round() as i32;
// let v = (hsv.value * 1000.0).round() as i32;

// println!("h: {}, s: {}, v: {}", h, s, v);
pub mod state;
pub mod default;