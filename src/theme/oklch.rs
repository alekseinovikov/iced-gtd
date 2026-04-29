use iced::Color;
use palette::{IntoColor, Oklch, Srgb};

pub fn oklch(l_pct: f32, c: f32, h_deg: f32) -> Color {
    let oklch = Oklch::new(l_pct / 100.0, c, h_deg);
    let srgb: Srgb = oklch.into_color();
    let clamped = srgb.into_format::<f32>();
    Color::from_rgb(
        clamped.red.clamp(0.0, 1.0),
        clamped.green.clamp(0.0, 1.0),
        clamped.blue.clamp(0.0, 1.0),
    )
}

pub fn oklch_a(l_pct: f32, c: f32, h_deg: f32, alpha: f32) -> Color {
    let mut color = oklch(l_pct, c, h_deg);
    color.a = alpha;
    color
}
