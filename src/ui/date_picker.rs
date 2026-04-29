use chrono::{Datelike, Duration, NaiveDate};
use iced::widget::{button, column, container, mouse_area, row, text, Space};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{App, Message};
use crate::date::{month_long, next_saturday, weekday_short};
use crate::models::AnchorKind;
use crate::styles;
use crate::ui::icons;

pub fn view(app: &App) -> Element<'_, Message> {
    let dp = match &app.date_picker {
        Some(dp) => dp.clone(),
        None => return Space::new().width(Length::Fixed(0.0)).into(),
    };
    let tokens = app.tokens;

    // Quick-row 2x2 grid
    let today = app.today;
    let tomorrow = today + Duration::days(1);
    let weekend = next_saturday(today);

    let qbtn = |icon, label: &str, day_hint: String, msg: Message| -> Element<'_, Message> {
        button(
            row![
                icons::colored(icon, 14, tokens.ink_3),
                text(label.to_string()).size(13).color(tokens.ink_2).width(Length::Fill),
                text(day_hint).size(11).color(tokens.ink_4),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([0.0, 10.0]))
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        .style(styles::button::quick_btn(tokens))
        .on_press(msg)
        .into()
    };

    let quick_grid = column![
        row![
            qbtn(icons::today(), "Today", format!("{}", today.day()), Message::DatePickerPick(Some(today))),
            qbtn(icons::clock(), "Tomorrow", weekday_short(tomorrow.weekday()).to_string(), Message::DatePickerPick(Some(tomorrow))),
        ]
        .spacing(4),
        row![
            qbtn(icons::weekend(), "This weekend", "Sat".to_string(), Message::DatePickerPick(Some(weekend))),
            qbtn(icons::someday(), "Someday", "—".to_string(), Message::DatePickerPick(None)),
        ]
        .spacing(4),
    ]
    .spacing(4);

    // Month nav
    let month_label = format!("{} {}", month_long(dp.view_month), dp.view_year);
    let nav = row![
        button(text("‹").size(14).color(tokens.ink_3))
            .padding(0)
            .width(Length::Fixed(24.0))
            .height(Length::Fixed(24.0))
            .style(styles::button::small_ghost(tokens))
            .on_press(Message::DatePickerNav(-1)),
        text(month_label).size(13).color(tokens.ink).width(Length::Fill),
        button(text("›").size(14).color(tokens.ink_3))
            .padding(0)
            .width(Length::Fixed(24.0))
            .height(Length::Fixed(24.0))
            .style(styles::button::small_ghost(tokens))
            .on_press(Message::DatePickerNav(1)),
    ]
    .align_y(Alignment::Center);

    // Day grid
    let dn_label = |s: &'static str| -> Element<'_, Message> {
        container(text(s).size(10.5).color(tokens.ink_4))
            .center_x(Length::Fill)
            .padding(Padding::from([4.0, 0.0]))
            .into()
    };

    let mut grid_rows = column![row![
        dn_label("Sun"),
        dn_label("Mon"),
        dn_label("Tue"),
        dn_label("Wed"),
        dn_label("Thu"),
        dn_label("Fri"),
        dn_label("Sat"),
    ]
    .spacing(2)];

    let first = NaiveDate::from_ymd_opt(dp.view_year, dp.view_month, 1).unwrap();
    let first_dow = first.weekday().num_days_from_sunday() as i32;
    let days_in_month = days_in(dp.view_year, dp.view_month);
    let prev_year = if dp.view_month == 1 { dp.view_year - 1 } else { dp.view_year };
    let prev_month = if dp.view_month == 1 { 12 } else { dp.view_month - 1 };
    let prev_days = days_in(prev_year, prev_month);
    let next_year = if dp.view_month == 12 { dp.view_year + 1 } else { dp.view_year };
    let next_month = if dp.view_month == 12 { 1 } else { dp.view_month + 1 };

    let mut cells: Vec<(i32, u32, u32, bool)> = Vec::new();
    // (year, month, day, muted)
    for i in 0..first_dow {
        let day = (prev_days as i32 - first_dow + 1 + i) as u32;
        cells.push((prev_year, prev_month, day, true));
    }
    for d in 1..=days_in_month {
        cells.push((dp.view_year, dp.view_month, d, false));
    }
    while cells.len() < 42 {
        let nxt = (cells.len() - days_in_month as usize - first_dow as usize + 1) as u32;
        cells.push((next_year, next_month, nxt, true));
    }

    for chunk in cells.chunks(7) {
        let mut r = row![].spacing(2);
        for (y, m, d, muted) in chunk {
            let date = NaiveDate::from_ymd_opt(*y, *m, *d).unwrap();
            let is_today = date == today;
            let is_selected = dp.initial_value == Some(date);
            let cell = button(text(d.to_string()).size(12).color(tokens.ink_2))
                .padding(0)
                .width(Length::Fill)
                .height(Length::Fixed(30.0))
                .style(styles::button::cal_day(tokens, *muted, is_today, is_selected))
                .on_press(Message::DatePickerPick(Some(date)));
            r = r.push(container(cell).width(Length::Fill));
        }
        grid_rows = grid_rows.push(r);
    }

    // Footer
    let footer = row![
        button(text("Clear").size(12).color(tokens.ink_2))
            .padding(Padding::from([0.0, 8.0]))
            .height(Length::Fixed(26.0))
            .style(styles::button::ghost(tokens))
            .on_press(Message::DatePickerPick(None)),
        Space::new().width(Length::Fill),
        button(text("Close").size(12).color(tokens.ink_2))
            .padding(Padding::from([0.0, 8.0]))
            .height(Length::Fixed(26.0))
            .style(styles::button::ghost(tokens))
            .on_press(Message::CloseDatePicker),
    ]
    .padding(Padding {
        top: 8.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    })
    .align_y(Alignment::Center);

    let body = column![quick_grid, nav, grid_rows, footer].spacing(8);

    let popover = container(body)
        .padding(12)
        .width(Length::Fixed(296.0))
        .style(styles::container::popover(tokens));

    // Anchor positioning via padding inside a fill-screen container
    let (top_pad, left_align, right_pad) = anchor_padding(dp.anchor, app.show_inspector);

    // Backdrop catches outside clicks
    let backdrop = mouse_area(
        container(Space::new().width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .on_press(Message::CloseDatePicker);

    let positioned = container(popover)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding {
            top: top_pad,
            right: right_pad,
            bottom: 0.0,
            left: 0.0,
        })
        .align_x(left_align)
        .align_y(iced::Alignment::Start);

    iced::widget::stack![Element::from(backdrop), Element::from(positioned)].into()
}

fn anchor_padding(anchor: AnchorKind, inspector_open: bool) -> (f32, iced::Alignment, f32) {
    match anchor {
        AnchorKind::Inspector => (
            120.0,
            iced::Alignment::End,
            if inspector_open { 340.0 } else { 24.0 },
        ),
        AnchorKind::BulkBar => (200.0, iced::Alignment::Center, 0.0),
        AnchorKind::MainCenter => (140.0, iced::Alignment::Center, 0.0),
    }
}

fn days_in(y: i32, m: u32) -> u32 {
    let next_y = if m == 12 { y + 1 } else { y };
    let next_m = if m == 12 { 1 } else { m + 1 };
    let first_next = NaiveDate::from_ymd_opt(next_y, next_m, 1).unwrap();
    let last = first_next - Duration::days(1);
    last.day()
}
