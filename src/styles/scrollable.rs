use iced::widget::{
    container,
    scrollable::{self, AutoScroll, Rail, Scroller},
};
use iced::{Background, Border, Color, Theme};

use crate::theme::Tokens;

fn transparent() -> Color {
    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
}

pub fn calm(tokens: Tokens) -> impl Fn(&Theme, scrollable::Status) -> scrollable::Style {
    move |_theme, _status| {
        let rail = Rail {
            background: Some(Background::Color(transparent())),
            border: Border::default(),
            scroller: Scroller {
                background: Background::Color(tokens.line),
                border: Border {
                    color: tokens.line,
                    width: 0.0,
                    radius: 4.0.into(),
                },
            },
        };
        scrollable::Style {
            container: container::Style {
                background: None,
                border: Border::default(),
                text_color: None,
                shadow: Default::default(),
                ..Default::default()
            },
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
            auto_scroll: AutoScroll {
                background: Background::Color(transparent()),
                border: Border::default(),
                shadow: Default::default(),
                icon: transparent(),
            },
        }
    }
}
