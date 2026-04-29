use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{App, Message};
use crate::styles;
use crate::ui::{icons, task_list};

pub fn view(app: &App) -> Element<'_, Message> {
    let tokens = app.tokens;
    let header_info = app.header_info();

    // Header
    let title_row = row![
        container(Space::new().width(Length::Fixed(10.0)).height(Length::Fixed(10.0)))
            .width(Length::Fixed(10.0))
            .height(Length::Fixed(10.0))
            .style(styles::container::accent_dot(tokens)),
        text(header_info.title.clone()).size(26).color(tokens.ink),
        text(header_info.meta.clone()).size(13).color(tokens.ink_3),
    ]
    .spacing(12)
    .align_y(Alignment::End);

    let header_main = if let Some(sub) = header_info.sub.clone() {
        column![title_row, text(sub).size(13).color(tokens.ink_3)].spacing(4)
    } else {
        column![title_row]
    };

    let new_task_btn = iced::widget::button(
        row![
            icons::colored(icons::plus(), 12, tokens.on_accent),
            text("New task").size(13).color(tokens.on_accent),
            container(text("\u{2318}N").size(11).color(tokens.on_accent))
                .padding(Padding::from([1.0, 5.0]))
                .style(styles::container::kbd_on_accent(tokens)),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([0.0, 12.0]))
    .height(Length::Fixed(30.0))
    .style(styles::button::primary(tokens))
    .on_press(Message::AddTaskAtView);

    let more_btn = iced::widget::button(icons::colored(icons::more(), 14, tokens.ink_2))
        .padding(0)
        .width(Length::Fixed(30.0))
        .height(Length::Fixed(30.0))
        .style(styles::button::ghost(tokens))
        .on_press(Message::Noop);

    let actions = row![more_btn, new_task_btn].spacing(6);

    let header_box = container(
        row![
            container(header_main).width(Length::Fill),
            container(actions).align_x(Alignment::End),
        ]
        .spacing(16)
        .align_y(Alignment::End),
    )
    .padding(Padding {
        top: 22.0,
        right: 32.0,
        bottom: 12.0,
        left: 32.0,
    });

    let header_border = container(Space::new().height(Length::Fixed(1.0)))
        .height(Length::Fixed(1.0))
        .width(Length::Fill)
        .style(styles::container::divider_soft(tokens));

    // Toolbar
    let chips = row![
        chip(tokens, "All", app.filter == crate::models::Filter::All, Message::ChipFilter(crate::models::Filter::All)),
        chip(tokens, "@work", app.filter == crate::models::Filter::Work, Message::ChipFilter(crate::models::Filter::Work)),
        chip(tokens, "@home", app.filter == crate::models::Filter::Home, Message::ChipFilter(crate::models::Filter::Home)),
        chip(tokens, "@errand", app.filter == crate::models::Filter::Errand, Message::ChipFilter(crate::models::Filter::Errand)),
    ]
    .spacing(8);

    let total_visible: usize = crate::app::build_groups(app)
        .iter()
        .map(|g| g.tasks.len())
        .sum();

    let toolbar = container(
        row![
            container(chips).width(Length::Fill),
            row![
                text(format!("{} tasks", total_visible))
                    .size(12.5)
                    .color(tokens.ink_4),
                container(text("\u{2318}F").size(10.5).color(tokens.ink_3))
                    .padding(Padding::from([1.0, 5.0]))
                    .style(styles::container::kbd(tokens)),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([8.0, 32.0]))
    .height(Length::Fixed(40.0));

    let toolbar_border = container(Space::new().height(Length::Fixed(1.0)))
        .height(Length::Fixed(1.0))
        .width(Length::Fill)
        .style(styles::container::divider_soft(tokens));

    // Task list
    let list = task_list::view(app);
    let list_pad = container(list).padding(Padding {
        top: 8.0,
        right: 24.0,
        bottom: 80.0,
        left: 24.0,
    });

    let scroll = scrollable(list_pad)
        .style(styles::scrollable::calm(tokens))
        .height(Length::Fill)
        .width(Length::Fill);

    column![header_box, header_border, toolbar, toolbar_border, scroll].into()
}

pub fn bulk_bar_overlay(app: &App) -> Element<'_, Message> {
    let tokens = app.tokens;
    let count = app.selection.len();
    let count_pill = container(text(count.to_string()).size(12).color(tokens.on_accent))
        .padding(Padding::from([2.0, 8.0]))
        .style(styles::container::count_pill(tokens));

    let sep = container(Space::new().width(Length::Fixed(1.0)))
        .width(Length::Fixed(1.0))
        .height(Length::Fixed(18.0))
        .style(styles::container::bulk_sep(tokens));

    let icon_btn = |handle, msg: Message| -> Element<'_, Message> {
        iced::widget::button(icons::colored(handle, 14, tokens.bg))
            .padding(0)
            .width(Length::Fixed(30.0))
            .height(Length::Fixed(30.0))
            .style(styles::button::bulk_button(tokens))
            .on_press(msg)
            .into()
    };

    let bar = container(
        row![
            count_pill,
            text("selected").size(13).color(tokens.bg),
            sep,
            icon_btn(icons::check(), Message::BulkComplete),
            icon_btn(icons::calendar(), Message::BulkScheduleSelected),
            icon_btn(icons::someday(), Message::Noop),
            icon_btn(icons::trash(), Message::BulkDelete),
            container(Space::new().width(Length::Fixed(1.0)))
                .width(Length::Fixed(1.0))
                .height(Length::Fixed(18.0))
                .style(styles::container::bulk_sep(tokens)),
            icon_btn(icons::x(), Message::ClearSelection),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding(Padding {
        top: 6.0,
        right: 8.0,
        bottom: 6.0,
        left: 16.0,
    })
    .style(styles::container::bulk_bar(tokens));

    let positioned = container(bar)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .align_y(iced::Alignment::End)
        .padding(Padding {
            top: 0.0,
            right: 0.0,
            bottom: 22.0,
            left: 0.0,
        });

    positioned.into()
}

fn chip<'a>(
    tokens: crate::theme::Tokens,
    label: &'a str,
    active: bool,
    msg: Message,
) -> Element<'a, Message> {
    let color = if active { tokens.bg } else { tokens.ink_2 };
    iced::widget::button(text(label).size(12).color(color))
        .padding(Padding::from([0.0, 9.0]))
        .height(Length::Fixed(24.0))
        .style(styles::button::chip(tokens, active))
        .on_press(msg)
        .into()
}
