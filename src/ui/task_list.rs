use iced::widget::{Space, column, container, row, text, text_input};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{App, Message};
use crate::styles;
use crate::ui::task_row;

pub fn view(app: &App) -> Element<'_, Message> {
    let tokens = app.tokens;
    let groups = crate::app::build_groups(app);

    let mut col = column![].spacing(0);
    for g in groups {
        if !g.title.is_empty() {
            col = col.push(section_divider(tokens, g.title.clone(), g.subtitle.clone()));
        }
        if g.tasks.is_empty() {
            if let Some(h) = g.empty_hint.clone() {
                col = col.push(empty_hint(tokens, h));
            }
        } else {
            for tid in &g.tasks {
                if let Some(t) = app.tasks.iter().find(|t| &t.id == tid) {
                    col = col.push(task_row::view(app, t));
                }
            }
        }
        if g.allow_add {
            col = col.push(new_task(app, g.key.clone(), g.add_placeholder.clone()));
        }
    }
    col.into()
}

fn section_divider<'a>(
    tokens: crate::theme::Tokens,
    title: String,
    subtitle: Option<String>,
) -> Element<'a, Message> {
    let mut children: Vec<Element<'a, Message>> = vec![
        text(title.to_uppercase())
            .size(11.5)
            .color(tokens.ink_4)
            .into(),
    ];
    if let Some(sub) = subtitle {
        children.push(text(sub).size(11.5).color(tokens.ink_3).into());
    }
    children.push(
        container(Space::new().height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(styles::container::divider_soft(tokens))
            .into(),
    );
    container(row(children).spacing(12).align_y(Alignment::Center))
        .padding(Padding {
            top: 18.0,
            right: 8.0,
            bottom: 6.0,
            left: 8.0,
        })
        .into()
}

fn empty_hint<'a>(tokens: crate::theme::Tokens, msg: String) -> Element<'a, Message> {
    container(text(msg).size(13).color(tokens.ink_4))
        .padding(Padding::from([20.0, 12.0]))
        .center_x(Length::Fill)
        .into()
}

fn new_task<'a>(app: &'a App, key: String, placeholder: String) -> Element<'a, Message> {
    let tokens = app.tokens;
    let buf = app.composer_buffers.get(&key).cloned().unwrap_or_default();
    let focused = app.composer_focused.as_deref() == Some(key.as_str());

    let dummy_check = container(
        Space::new()
            .width(Length::Fixed(18.0))
            .height(Length::Fixed(18.0)),
    )
    .width(Length::Fixed(18.0))
    .height(Length::Fixed(18.0));

    let key_for_input = key.clone();
    let key_for_submit = key.clone();
    let key_for_focus = key.clone();
    let id_str = format!("composer-{}", key);

    let input = text_input(&placeholder, &buf)
        .id(iced::widget::Id::from(id_str))
        .on_input(move |s| Message::ComposerChanged(key_for_input.clone(), s))
        .on_submit(Message::ComposerSubmit(key_for_submit.clone()))
        .padding(0)
        .size(14)
        .style(styles::text_input::flat(tokens));

    let mut row_children: Vec<Element<'a, Message>> = vec![
        Space::new().width(Length::Fixed(24.0)).into(),
        dummy_check.into(),
        input.into(),
    ];

    if focused {
        row_children.push(
            container(
                row![
                    container(text("\u{21b5}").size(11).color(tokens.ink_4))
                        .padding(Padding::from([1.0, 5.0]))
                        .style(styles::container::kbd(tokens)),
                    text("save").size(11).color(tokens.ink_4),
                    text("\u{00b7}").size(11).color(tokens.ink_4),
                    container(text("esc").size(11).color(tokens.ink_4))
                        .padding(Padding::from([1.0, 5.0]))
                        .style(styles::container::kbd(tokens)),
                    text("cancel").size(11).color(tokens.ink_4),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .into(),
        );
    }

    let inner = row(row_children).spacing(10).align_y(Alignment::Center);

    let pane = if focused {
        container(inner)
            .padding(Padding::from([app.density.pad_y() as f32, 8.0]))
            .style(styles::container::new_task_focused(tokens))
            .width(Length::Fill)
    } else {
        container(inner)
            .padding(Padding::from([app.density.pad_y() as f32, 8.0]))
            .style(styles::container::new_task_idle(tokens))
            .width(Length::Fill)
    };

    iced::widget::mouse_area(pane)
        .on_press(Message::ComposerFocus(Some(key_for_focus)))
        .into()
}
