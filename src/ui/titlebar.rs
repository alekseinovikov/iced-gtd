use iced::widget::{Space, button, column, container, mouse_area, row, text};
use iced::{Element, Length, Padding};

use crate::app::{App, Message};
use crate::models::ThemeMode;
use crate::styles;
use crate::ui::icons;

pub fn view(app: &App) -> Element<'_, Message> {
    let tokens = app.tokens;

    let win_dot = |on_press: Option<Message>| -> Element<'_, Message> {
        let dot = container(
            Space::new()
                .width(Length::Fixed(13.0))
                .height(Length::Fixed(13.0)),
        )
        .width(Length::Fixed(13.0))
        .height(Length::Fixed(13.0))
        .style(styles::container::win_dot(tokens));
        let mut area = mouse_area(dot);
        if let Some(msg) = on_press {
            area = area.on_press(msg);
        }
        Element::from(area)
    };

    let icon_btn = |handle, msg: Message| -> Element<'_, Message> {
        let inner = icons::colored(handle, 14, tokens.ink_3);
        button(
            container(inner)
                .width(Length::Fixed(26.0))
                .height(Length::Fixed(26.0))
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center),
        )
        .on_press(msg)
        .padding(0)
        .width(Length::Fixed(26.0))
        .height(Length::Fixed(26.0))
        .style(styles::button::titlebar_icon(tokens))
        .into()
    };

    let theme_icon_handle = if matches!(app.theme_mode, ThemeMode::Light) {
        icons::moon()
    } else {
        icons::sun()
    };

    let next_theme = match app.theme_mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };

    let left = row![
        win_dot(Some(Message::WindowClose)),
        win_dot(Some(Message::WindowMinimize)),
        win_dot(None),
        Space::new().width(Length::Fixed(6.0)),
        icon_btn(icons::sidebar(), Message::ToggleSidebar)
    ]
    .spacing(6.0)
    .align_y(iced::Alignment::Center);

    let snowflake = text("\u{2744} ").size(13).color(tokens.accent);
    let title_bold = text("IcedGTD").size(13).color(tokens.ink).font(iced::Font {
        weight: iced::font::Weight::Semibold,
        ..Default::default()
    });
    let title_dim = text("  \u{00b7}  built with Iced.rs")
        .size(12)
        .color(tokens.ink_3);
    let center = row![snowflake, title_bold, title_dim].align_y(iced::Alignment::Center);

    let right = row![
        icon_btn(theme_icon_handle, Message::SetTheme(next_theme)),
        icon_btn(icons::inspector(), Message::ToggleInspector),
    ]
    .spacing(6.0)
    .align_y(iced::Alignment::Center);

    let bar = row![
        container(left)
            .width(Length::FillPortion(1))
            .align_x(iced::Alignment::Start),
        container(center)
            .width(Length::Shrink)
            .center_x(Length::Shrink),
        container(right)
            .width(Length::FillPortion(1))
            .align_x(iced::Alignment::End),
    ]
    .align_y(iced::Alignment::Center)
    .height(Length::Fixed(38.0));

    let drag_area = mouse_area(
        container(bar)
            .padding(Padding::from([0, 12]))
            .width(Length::Fill)
            .height(Length::Fixed(38.0))
            .style(styles::container::titlebar(tokens)),
    )
    .on_press(Message::DragWindow);

    let bottom_border = container(Space::new().height(Length::Fixed(1.0)))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(styles::container::divider(tokens));

    column![drag_area, bottom_border].into()
}
