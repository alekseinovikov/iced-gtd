pub mod oklch;

use iced::{Color, Shadow, Vector};
use oklch::{oklch, oklch_a};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tokens {
    pub bg: Color,
    pub bg_2: Color,
    pub bg_3: Color,
    pub surface: Color,
    pub line: Color,
    pub line_soft: Color,
    pub ink: Color,
    pub ink_2: Color,
    pub ink_3: Color,
    pub ink_4: Color,
    pub accent: Color,
    pub accent_2: Color,
    pub accent_soft: Color,
    pub accent_ink: Color,
    pub accent_border: Color,
    pub danger: Color,
    pub warn: Color,
    pub on_accent: Color,
    pub bulk_button_hover: Color,
    pub bulk_sep: Color,
    pub shadow_1: Shadow,
    pub shadow_2: Shadow,
    pub shadow_3: Shadow,
    pub is_dark: bool,
}

impl Tokens {
    pub fn light() -> Self {
        let accent = oklch(72.0, 0.12, 200.0);
        let line = oklch(91.0, 0.006, 220.0);
        let bg = oklch(99.0, 0.003, 220.0);
        Self {
            bg,
            bg_2: oklch(97.0, 0.005, 220.0),
            bg_3: oklch(94.5, 0.006, 220.0),
            surface: oklch(100.0, 0.0, 0.0),
            line,
            line_soft: oklch(94.0, 0.005, 220.0),
            ink: oklch(20.0, 0.01, 240.0),
            ink_2: oklch(40.0, 0.012, 240.0),
            ink_3: oklch(58.0, 0.012, 240.0),
            ink_4: oklch(72.0, 0.01, 240.0),
            accent,
            accent_2: oklch(82.0, 0.09, 200.0),
            accent_soft: oklch(95.0, 0.04, 200.0),
            accent_ink: oklch(35.0, 0.08, 220.0),
            accent_border: oklch_a(72.0, 0.12, 200.0, 0.35),
            danger: oklch(62.0, 0.18, 25.0),
            warn: oklch(72.0, 0.14, 70.0),
            on_accent: oklch(20.0, 0.04, 220.0),
            bulk_button_hover: oklch_a(99.0, 0.003, 220.0, 0.15),
            bulk_sep: oklch_a(99.0, 0.003, 220.0, 0.2),
            shadow_1: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.04),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            shadow_2: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.07),
                offset: Vector::new(0.0, 6.0),
                blur_radius: 18.0,
            },
            shadow_3: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.13),
                offset: Vector::new(0.0, 18.0),
                blur_radius: 50.0,
            },
            is_dark: false,
        }
    }

    pub fn dark() -> Self {
        let accent = oklch(78.0, 0.13, 200.0);
        Self {
            bg: oklch(15.0, 0.008, 240.0),
            bg_2: oklch(18.0, 0.009, 240.0),
            bg_3: oklch(21.0, 0.01, 240.0),
            surface: oklch(20.0, 0.009, 240.0),
            line: oklch(28.0, 0.01, 240.0),
            line_soft: oklch(24.0, 0.009, 240.0),
            ink: oklch(95.0, 0.005, 220.0),
            ink_2: oklch(82.0, 0.008, 220.0),
            ink_3: oklch(64.0, 0.01, 220.0),
            ink_4: oklch(48.0, 0.01, 220.0),
            accent,
            accent_2: oklch(68.0, 0.11, 200.0),
            accent_soft: oklch(28.0, 0.06, 210.0),
            accent_ink: oklch(88.0, 0.09, 200.0),
            accent_border: oklch_a(78.0, 0.13, 200.0, 0.35),
            danger: oklch(62.0, 0.18, 25.0),
            warn: oklch(72.0, 0.14, 70.0),
            on_accent: oklch(20.0, 0.04, 220.0),
            bulk_button_hover: oklch_a(15.0, 0.008, 240.0, 0.15),
            bulk_sep: oklch_a(15.0, 0.008, 240.0, 0.2),
            shadow_1: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            shadow_2: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                offset: Vector::new(0.0, 6.0),
                blur_radius: 18.0,
            },
            shadow_3: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                offset: Vector::new(0.0, 18.0),
                blur_radius: 50.0,
            },
            is_dark: true,
        }
    }
}
