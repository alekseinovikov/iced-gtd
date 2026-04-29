use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{App, Message};
use crate::models::View;
use crate::styles;
use crate::ui::{icons, progress_ring};

pub fn view(app: &App) -> Element<'_, Message> {
    let tokens = app.tokens;
    let counts = app.counts();

    // Search box
    let search = container(
        row![
            icons::colored(icons::search(), 14, tokens.ink_3),
            text_input("Search", &app.search_query)
                .id(iced::widget::Id::new("sidebar-search"))
                .on_input(Message::SidebarSearchChanged)
                .padding(0)
                .size(13)
                .style(styles::text_input::search(tokens)),
            kbd(tokens, "⌘K"),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .padding(Padding::from([0.0, 10.0]))
    .height(Length::Fixed(30.0))
    .center_y(Length::Fixed(30.0))
    .style(styles::container::search_box(tokens));

    let search_section = container(search).padding(Padding {
        top: 10.0,
        right: 12.0,
        bottom: 8.0,
        left: 12.0,
    });

    // Top nav
    let nav_items = [
        (View::Inbox, "Inbox", icons::inbox(), Some(counts.inbox)),
        (View::Today, "Today", icons::today(), Some(counts.today)),
        (
            View::Upcoming,
            "Upcoming",
            icons::upcoming(),
            Some(counts.upcoming),
        ),
        (
            View::Anytime,
            "Anytime",
            icons::anytime(),
            Some(counts.anytime),
        ),
        (View::Someday, "Someday", icons::someday(), None),
    ];

    let mut nav_col = column![].spacing(2);
    for (v, label, icon, cnt) in nav_items {
        let active = app.view == v && app.active_project.is_none();
        nav_col = nav_col.push(nav_item(
            tokens,
            icon,
            label,
            cnt,
            active,
            Message::SelectView(v),
        ));
    }

    // Areas section header
    let areas_header = container(
        row![
            text("AREAS")
                .size(10.5)
                .color(tokens.ink_4)
                .width(Length::Fill),
            // The "+ new area" button is decorative for v1
            container(icons::colored(icons::plus(), 12, tokens.ink_4)).padding(2)
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding {
        top: 14.0,
        right: 12.0,
        bottom: 6.0,
        left: 12.0,
    });

    let mut areas_col = column![].spacing(0);
    for area in &app.areas {
        let collapsed = app.collapsed_areas.contains(&area.id);
        let chev = if collapsed {
            icons::chevron_right()
        } else {
            icons::chevron_down()
        };

        let header = button(
            row![
                icons::colored(chev, 10, tokens.ink_4),
                container(
                    Space::new()
                        .width(Length::Fixed(8.0))
                        .height(Length::Fixed(8.0))
                )
                .width(Length::Fixed(8.0))
                .height(Length::Fixed(8.0))
                .style(styles::container::area_dot(tokens)),
                text(&area.name).size(12.5).color(tokens.ink_2),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([0.0, 8.0]))
        .height(Length::Fixed(28.0))
        .width(Length::Fill)
        .on_press(Message::ToggleAreaCollapse(area.id.clone()))
        .style(styles::button::area_header(tokens));

        areas_col = areas_col.push(header);

        if !collapsed {
            let mut proj_col = column![].spacing(0);
            for p in &area.projects {
                let total = p.tasks.max(1) as f32;
                let progress = (p.done as f32) / total;
                let active = app.active_project.as_deref() == Some(&p.id);
                let open_count = p.tasks.saturating_sub(p.done);
                let count_color = if active {
                    tokens.accent_ink
                } else {
                    tokens.ink_4
                };
                let item = button(
                    row![
                        progress_ring::ring(progress, active, tokens),
                        text(&p.name)
                            .size(13)
                            .color(if active {
                                tokens.accent_ink
                            } else {
                                tokens.ink_2
                            })
                            .width(Length::Fill),
                        text(open_count.to_string()).size(11.5).color(count_color),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                )
                .padding(Padding::from([0.0, 8.0]))
                .height(Length::Fixed(28.0))
                .width(Length::Fill)
                .on_press(Message::SelectProject(p.id.clone()))
                .style(styles::button::proj_item(tokens, active));
                proj_col = proj_col.push(item);
            }
            let proj_indent = container(proj_col).padding(Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 22.0,
            });
            areas_col = areas_col.push(proj_indent);
        }
    }

    let archive_header =
        container(text("ARCHIVE").size(10.5).color(tokens.ink_4)).padding(Padding {
            top: 14.0,
            right: 12.0,
            bottom: 6.0,
            left: 12.0,
        });

    let archive_items = column![
        nav_item(
            tokens,
            icons::log(),
            "Logbook",
            None,
            app.view == View::Logbook && app.active_project.is_none(),
            Message::SelectView(View::Logbook)
        ),
        nav_item(
            tokens,
            icons::trash(),
            "Trash",
            None,
            app.view == View::Trash && app.active_project.is_none(),
            Message::SelectView(View::Trash)
        ),
    ]
    .spacing(2);

    let scroll_inner = column![
        nav_col,
        areas_header,
        areas_col,
        archive_header,
        archive_items,
    ]
    .spacing(0);

    let scroll_pad = container(scroll_inner).padding(Padding {
        top: 4.0,
        right: 8.0,
        bottom: 16.0,
        left: 8.0,
    });

    let scroll = scrollable(scroll_pad)
        .style(styles::scrollable::calm(tokens))
        .height(Length::Fill)
        .width(Length::Fill);

    // Footer
    let avatar = container(
        text("SK")
            .size(12)
            .color(tokens.on_accent)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            }),
    )
    .width(Length::Fixed(28.0))
    .height(Length::Fixed(28.0))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(styles::container::avatar_grad(tokens));

    let name_block = column![
        text("Sara Kjartansdóttir").size(13).color(tokens.ink),
        text("Synced · just now").size(11).color(tokens.ink_4),
    ]
    .spacing(2);

    let more_btn = button(icons::colored(icons::more(), 14, tokens.ink_3))
        .padding(0)
        .width(Length::Fixed(26.0))
        .height(Length::Fixed(26.0))
        .style(styles::button::titlebar_icon(tokens))
        .on_press(Message::Noop);

    let footer_top = container(Space::new().height(Length::Fixed(1.0)))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(styles::container::divider(tokens));

    let footer_inner = container(
        row![
            avatar,
            name_block,
            Space::new().width(Length::Fill),
            more_btn,
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .padding(Padding {
        top: 10.0,
        right: 12.0,
        bottom: 10.0,
        left: 12.0,
    });

    let footer = column![footer_top, footer_inner];

    let column_contents = column![search_section, scroll, footer];

    let pane = container(column_contents)
        .width(Length::Fixed(248.0))
        .height(Length::Fill)
        .style(styles::container::sidebar(tokens));

    // Right border
    let right_border = container(Space::new().width(Length::Fixed(1.0)))
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(styles::container::divider(tokens));

    row![pane, right_border].into()
}

fn nav_item<'a>(
    tokens: crate::theme::Tokens,
    icon_handle: iced::widget::svg::Handle,
    label: &'a str,
    count: Option<u32>,
    active: bool,
    msg: Message,
) -> Element<'a, Message> {
    let icon_color = if active { tokens.accent } else { tokens.ink_3 };
    let label_color = if active {
        if tokens.is_dark {
            tokens.accent
        } else {
            tokens.accent_ink
        }
    } else {
        tokens.ink_2
    };
    let count_color = if active {
        if tokens.is_dark {
            tokens.accent
        } else {
            tokens.accent_ink
        }
    } else {
        tokens.ink_3
    };
    let mut row_children = vec![
        icons::colored(icon_handle, 16, icon_color),
        text(label)
            .size(13.5)
            .color(label_color)
            .width(Length::Fill)
            .into(),
    ];
    if let Some(c) = count
        && c > 0
    {
        row_children.push(text(c.to_string()).size(11.5).color(count_color).into());
    }
    button(row(row_children).spacing(8).align_y(Alignment::Center))
        .padding(Padding::from([0.0, 8.0]))
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        .on_press(msg)
        .style(styles::button::nav_item(tokens, active))
        .into()
}

fn kbd<'a>(tokens: crate::theme::Tokens, label: &'a str) -> Element<'a, Message> {
    container(text(label).size(10.5).color(tokens.ink_3))
        .padding(Padding::from([1.0, 5.0]))
        .style(styles::container::kbd(tokens))
        .into()
}
