use iced::widget::button;
use iced::{Background, Border, Color, Theme};

use crate::theme::Tokens;

fn transparent() -> Color {
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    }
}

pub fn primary(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => tokens.accent_2,
            _ => tokens.accent,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tokens.on_accent,
            border: Border {
                color: bg,
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: tokens.shadow_1,
            ..button::Style::default()
        }
    }
}

pub fn neutral(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = match status {
            button::Status::Hovered => tokens.bg_3,
            button::Status::Pressed => tokens.bg_3,
            _ => tokens.surface,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tokens.ink,
            border: Border {
                color: tokens.line,
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: tokens.shadow_1,
            ..button::Style::default()
        }
    }
}

pub fn ghost(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = match status {
            button::Status::Hovered => Background::Color(tokens.bg_3),
            _ => Background::Color(transparent()),
        };
        let text_color = match status {
            button::Status::Hovered => tokens.ink,
            _ => tokens.ink_2,
        };
        button::Style {
            background: Some(bg),
            text_color,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 7.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn titlebar_icon(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, fg) = match status {
            button::Status::Hovered => (tokens.bg_3, tokens.ink),
            _ => (transparent(), tokens.ink_3),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: fg,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn nav_item(tokens: Tokens, active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, fg) = if active {
            let fg = if tokens.is_dark {
                tokens.accent
            } else {
                tokens.accent_ink
            };
            (tokens.accent_soft, fg)
        } else {
            match status {
                button::Status::Hovered => (tokens.bg_3, tokens.ink),
                _ => (transparent(), tokens.ink_2),
            }
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: fg,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 7.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn area_header(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = match status {
            button::Status::Hovered => tokens.bg_3,
            _ => transparent(),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tokens.ink_2,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn proj_item(tokens: Tokens, active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, fg) = if active {
            let fg = if tokens.is_dark {
                tokens.accent
            } else {
                tokens.accent_ink
            };
            (tokens.accent_soft, fg)
        } else {
            match status {
                button::Status::Hovered => (tokens.bg_3, tokens.ink),
                _ => (transparent(), tokens.ink_2),
            }
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: fg,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn chip(tokens: Tokens, active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, fg, border) = if active {
            (tokens.ink, tokens.bg, tokens.ink)
        } else {
            match status {
                button::Status::Hovered => (tokens.bg_3, tokens.ink_2, tokens.line),
                _ => (tokens.surface, tokens.ink_2, tokens.line),
            }
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: fg,
            border: Border {
                color: border,
                width: 1.0,
                radius: 999.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn small_ghost(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = match status {
            button::Status::Hovered => tokens.bg_3,
            _ => transparent(),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tokens.ink_2,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 5.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn checkbox_btn(
    tokens: Tokens,
    checked: bool,
    deadline: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        if checked {
            button::Style {
                background: Some(Background::Color(tokens.accent)),
                text_color: tokens.on_accent,
                border: Border {
                    color: tokens.accent,
                    width: 1.5,
                    radius: 4.0.into(),
                },
                shadow: Default::default(),
                ..button::Style::default()
            }
        } else {
            let (bg, border) = match status {
                button::Status::Hovered => (tokens.bg_3, tokens.ink_2),
                _ => (
                    transparent(),
                    if deadline { tokens.warn } else { tokens.ink_4 },
                ),
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: tokens.ink_4,
                border: Border {
                    color: border,
                    width: 1.5,
                    radius: 4.0.into(),
                },
                shadow: Default::default(),
                ..button::Style::default()
            }
        }
    }
}

pub fn cal_day(
    tokens: Tokens,
    muted: bool,
    is_today: bool,
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        if selected {
            button::Style {
                background: Some(Background::Color(tokens.accent)),
                text_color: tokens.on_accent,
                border: Border {
                    color: tokens.accent,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Default::default(),
                ..button::Style::default()
            }
        } else if is_today {
            button::Style {
                background: Some(Background::Color(transparent())),
                text_color: if tokens.is_dark {
                    tokens.accent
                } else {
                    tokens.accent_ink
                },
                border: Border {
                    color: tokens.accent,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Default::default(),
                ..button::Style::default()
            }
        } else {
            let (bg, fg) = match status {
                button::Status::Hovered => (tokens.bg_3, tokens.ink),
                _ => (
                    transparent(),
                    if muted { tokens.ink_4 } else { tokens.ink_2 },
                ),
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: transparent(),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Default::default(),
                ..button::Style::default()
            }
        }
    }
}

pub fn quick_btn(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, fg) = match status {
            button::Status::Hovered => (tokens.bg_3, tokens.ink),
            _ => (transparent(), tokens.ink_2),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: fg,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn bulk_button(tokens: Tokens) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = match status {
            button::Status::Hovered => tokens.bulk_button_hover,
            _ => transparent(),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tokens.bg,
            border: Border {
                color: transparent(),
                width: 0.0,
                radius: 999.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}

pub fn task_row(
    tokens: Tokens,
    selected: bool,
    hovered_state: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = if selected {
            tokens.accent_soft
        } else {
            match status {
                button::Status::Hovered => tokens.bg_2,
                _ if hovered_state => tokens.bg_2,
                _ => transparent(),
            }
        };
        let border_color = if selected {
            tokens.accent_border
        } else {
            transparent()
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tokens.ink,
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Default::default(),
            ..button::Style::default()
        }
    }
}
