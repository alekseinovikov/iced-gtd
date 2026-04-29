use iced::widget::container;
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

pub fn window_shell(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 14.0.into(),
        },
        text_color: Some(tokens.ink),
        shadow: tokens.shadow_3,
        ..Default::default()
    }
}

pub fn titlebar(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg_2)),
        border: Border {
            color: tokens.line,
            width: 0.0,
            radius: iced::border::Radius {
                top_left: 14.0,
                top_right: 14.0,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
        },
        text_color: Some(tokens.ink),
        ..Default::default()
    }
}

pub fn titlebar_bottom_border(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.line)),
        border: Border::default(),
        ..Default::default()
    }
}

pub fn sidebar(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg_2)),
        border: Border {
            color: tokens.line,
            width: 0.0,
            radius: 0.0.into(),
        },
        text_color: Some(tokens.ink),
        ..Default::default()
    }
}

pub fn sidebar_right_border(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.line)),
        border: Border::default(),
        ..Default::default()
    }
}

pub fn main_pane(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg)),
        border: Border::default(),
        text_color: Some(tokens.ink),
        ..Default::default()
    }
}

pub fn inspector(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg_2)),
        border: Border::default(),
        text_color: Some(tokens.ink),
        ..Default::default()
    }
}

pub fn search_box(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.surface)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 7.0.into(),
        },
        text_color: Some(tokens.ink_3),
        ..Default::default()
    }
}

pub fn divider(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.line)),
        ..Default::default()
    }
}

pub fn divider_soft(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.line_soft)),
        ..Default::default()
    }
}

pub fn accent_dot(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.accent)),
        border: Border {
            color: tokens.accent,
            width: 0.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

pub fn avatar_grad(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    // Approximation of linear-gradient(135deg, accent, accent_2): pick accent_2 since
    // iced container backgrounds support gradients via Background::Gradient.
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.accent_2)),
        border: Border {
            color: tokens.accent_2,
            width: 0.0,
            radius: 999.0.into(),
        },
        text_color: Some(tokens.on_accent),
        ..Default::default()
    }
}

pub fn win_dot(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg_3)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 999.0.into(),
        },
        ..Default::default()
    }
}

pub fn kbd(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg_2)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: Some(tokens.ink_3),
        ..Default::default()
    }
}

pub fn kbd_on_accent(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.accent_2)),
        border: Border {
            color: tokens.accent_2,
            width: 0.0,
            radius: 4.0.into(),
        },
        text_color: Some(tokens.on_accent),
        ..Default::default()
    }
}

pub fn tag_pill(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg_3)),
        border: Border {
            color: tokens.line_soft,
            width: 1.0,
            radius: 999.0.into(),
        },
        text_color: Some(tokens.ink_2),
        ..Default::default()
    }
}

pub fn checklist_mini(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bg_2)),
        border: Border {
            color: tokens.line_soft,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: Some(tokens.ink_3),
        ..Default::default()
    }
}

pub fn area_dot(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.accent)),
        border: Border {
            color: tokens.accent,
            width: 0.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

pub fn proj_left_guide(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.line_soft)),
        ..Default::default()
    }
}

pub fn new_task_idle(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(transparent())),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(tokens.ink_4),
        ..Default::default()
    }
}

pub fn new_task_focused(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.surface)),
        border: Border {
            color: tokens.accent,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(tokens.ink),
        shadow: tokens.shadow_1,
        ..Default::default()
    }
}

pub fn popover(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.surface)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 12.0.into(),
        },
        text_color: Some(tokens.ink),
        shadow: tokens.shadow_3,
        ..Default::default()
    }
}

pub fn bulk_bar(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.ink)),
        border: Border {
            color: tokens.ink,
            width: 0.0,
            radius: 999.0.into(),
        },
        text_color: Some(tokens.bg),
        shadow: tokens.shadow_3,
        ..Default::default()
    }
}

pub fn count_pill(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.accent)),
        border: Border {
            color: tokens.accent,
            width: 0.0,
            radius: 999.0.into(),
        },
        text_color: Some(tokens.on_accent),
        ..Default::default()
    }
}

pub fn bulk_sep(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.bulk_sep)),
        ..Default::default()
    }
}

pub fn notes(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.surface)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(tokens.ink_2),
        ..Default::default()
    }
}

pub fn checklist_item(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(transparent())),
        border: Border {
            color: transparent(),
            width: 0.0,
            radius: 5.0.into(),
        },
        text_color: Some(tokens.ink),
        ..Default::default()
    }
}

pub fn proj_dot(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.accent)),
        border: Border {
            color: tokens.accent,
            width: 0.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

pub fn drop_indicator(tokens: Tokens) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.accent)),
        border: Border {
            color: tokens.accent,
            width: 0.0,
            radius: 1.0.into(),
        },
        ..Default::default()
    }
}
