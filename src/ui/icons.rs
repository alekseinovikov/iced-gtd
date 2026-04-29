use std::borrow::Cow;

use iced::widget::svg::{self, Handle};
use iced::widget::{container, Svg};
use iced::{Color, Element, Length, Theme};

macro_rules! icon_handle {
    ($name:ident, $path:literal) => {
        pub fn $name() -> Handle {
            static BYTES: &[u8] = include_bytes!(concat!("../../assets/icons/", $path));
            Handle::from_memory(Cow::Borrowed(BYTES))
        }
    };
}

icon_handle!(inbox, "inbox.svg");
icon_handle!(today, "today.svg");
icon_handle!(upcoming, "upcoming.svg");
icon_handle!(anytime, "anytime.svg");
icon_handle!(someday, "someday.svg");
icon_handle!(log, "log.svg");
icon_handle!(trash, "trash.svg");
icon_handle!(chevron_down, "chevron-down.svg");
icon_handle!(chevron_right, "chevron-right.svg");
icon_handle!(plus, "plus.svg");
icon_handle!(search, "search.svg");
icon_handle!(check, "check.svg");
icon_handle!(dash, "dash.svg");
icon_handle!(flag, "flag.svg");
icon_handle!(repeat, "repeat.svg");
icon_handle!(grip, "grip.svg");
icon_handle!(sun, "sun.svg");
icon_handle!(moon, "moon.svg");
icon_handle!(sidebar, "sidebar.svg");
icon_handle!(inspector, "inspector.svg");
icon_handle!(calendar, "calendar.svg");
icon_handle!(clock, "clock.svg");
icon_handle!(weekend, "weekend.svg");
icon_handle!(archive, "archive.svg");
icon_handle!(more, "more.svg");
icon_handle!(x, "x.svg");

pub fn colored<'a, Message: 'a>(handle: Handle, size: u16, color: Color) -> Element<'a, Message> {
    let s = size as f32;
    let svg = Svg::new(handle).style(move |_theme: &Theme, _status| svg::Style {
        color: Some(color),
    });
    container(svg.width(Length::Fixed(s)).height(Length::Fixed(s)))
        .width(Length::Fixed(s))
        .height(Length::Fixed(s))
        .into()
}

pub fn plain<'a, Message: 'a>(handle: Handle, size: u16) -> Element<'a, Message> {
    let s = size as f32;
    container(Svg::new(handle).width(Length::Fixed(s)).height(Length::Fixed(s)))
        .width(Length::Fixed(s))
        .height(Length::Fixed(s))
        .into()
}
