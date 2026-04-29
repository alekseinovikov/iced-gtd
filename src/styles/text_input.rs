use iced::widget::text_input;
use iced::{Background, Border, Color, Theme};

use crate::theme::Tokens;

fn transparent() -> Color {
    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
}

pub fn search(tokens: Tokens) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_theme, _status| text_input::Style {
        background: Background::Color(transparent()),
        border: Border {
            color: transparent(),
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: tokens.ink_3,
        placeholder: tokens.ink_4,
        value: tokens.ink,
        selection: tokens.accent_soft,
    }
}

pub fn flat(tokens: Tokens) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_theme, _status| text_input::Style {
        background: Background::Color(transparent()),
        border: Border {
            color: transparent(),
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: tokens.ink_3,
        placeholder: tokens.ink_4,
        value: tokens.ink,
        selection: tokens.accent_soft,
    }
}

pub fn notes_box(tokens: Tokens) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_theme, _status| text_input::Style {
        background: Background::Color(tokens.surface),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        icon: tokens.ink_3,
        placeholder: tokens.ink_4,
        value: tokens.ink_2,
        selection: tokens.accent_soft,
    }
}
