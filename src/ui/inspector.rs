use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{App, Message};
use crate::date::format_date;
use crate::models::{DateField, DateTarget};
use crate::styles;
use crate::ui::icons;

pub fn view(app: &App) -> Element<'_, Message> {
    let tokens = app.tokens;
    let task = match app.open_task_ref() {
        Some(t) => t,
        None => return Space::new().width(Length::Fixed(0.0)).into(),
    };

    let project_info = task.project.as_ref().and_then(|pid| {
        app.areas.iter().find_map(|a| {
            a.projects
                .iter()
                .find(|p| &p.id == pid)
                .map(|p| (p.clone(), a.clone()))
        })
    });

    // Crumbs
    let dot = container(Space::new().width(Length::Fixed(6.0)).height(Length::Fixed(6.0)))
        .width(Length::Fixed(6.0))
        .height(Length::Fixed(6.0))
        .style(styles::container::accent_dot(tokens));

    let mut crumbs_children: Vec<Element<'_, Message>> = vec![dot.into()];
    if let Some((p, a)) = project_info.clone() {
        crumbs_children.push(text(a.name.to_uppercase()).size(11.5).color(tokens.ink_4).into());
        crumbs_children.push(text("/").size(11.5).color(tokens.ink_4).into());
        crumbs_children.push(text(p.name.to_uppercase()).size(11.5).color(tokens.ink_4).into());
    } else {
        crumbs_children.push(text("INBOX").size(11.5).color(tokens.ink_4).into());
    }
    let crumbs = row(crumbs_children).spacing(6).align_y(Alignment::Center);

    let close_btn = button(icons::colored(icons::x(), 12, tokens.ink_3))
        .padding(0)
        .width(Length::Fixed(26.0))
        .height(Length::Fixed(26.0))
        .style(styles::button::titlebar_icon(tokens))
        .on_press(Message::InspectorClose);

    let header_top = row![container(crumbs).width(Length::Fill), close_btn]
        .align_y(Alignment::Center);

    let title_input = text_input("Untitled", &app.inspector_title_buf)
        .id(iced::widget::Id::new("inspector-title"))
        .on_input(Message::EditTaskTitle)
        .on_submit(Message::CommitTaskTitle)
        .padding(0)
        .size(18)
        .style(styles::text_input::flat(tokens));

    let header = container(column![header_top, title_input].spacing(8))
        .padding(Padding {
            top: 18.0,
            right: 20.0,
            bottom: 12.0,
            left: 20.0,
        });

    let header_border = container(Space::new().height(Length::Fixed(1.0)))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(styles::container::divider_soft(tokens));

    // Body fields
    let when_btn = button(
        row![
            icons::colored(icons::calendar(), 12, tokens.ink_2),
            text(match task.when {
                Some(d) => format_date(d, app.today),
                None => "Someday".to_string(),
            })
            .size(13)
            .color(tokens.ink_2),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([0.0, 8.0]))
    .height(Length::Fixed(26.0))
    .style(styles::button::ghost(tokens))
    .on_press(Message::OpenDatePicker {
        target: DateTarget::Single(task.id.clone()),
        field: DateField::When,
        anchor: crate::models::AnchorKind::Inspector,
        initial: task.when,
    });

    let deadline_btn = button(
        row![
            icons::colored(icons::flag(), 12, tokens.ink_2),
            text(match task.deadline {
                Some(d) => format_date(d, app.today),
                None => "No deadline".to_string(),
            })
            .size(13)
            .color(if task.deadline.is_some() { tokens.ink_2 } else { tokens.ink_4 }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([0.0, 8.0]))
    .height(Length::Fixed(26.0))
    .style(styles::button::ghost(tokens))
    .on_press(Message::OpenDatePicker {
        target: DateTarget::Single(task.id.clone()),
        field: DateField::Deadline,
        anchor: crate::models::AnchorKind::Inspector,
        initial: task.deadline,
    });

    let repeat_value: Element<'_, Message> = if let Some(r) = task.repeat.clone() {
        row![
            icons::colored(icons::repeat(), 12, tokens.ink_2),
            text(r).size(13).color(tokens.ink_2),
        ]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
    } else {
        text("Never").size(13).color(tokens.ink_4).into()
    };

    let project_value: Element<'_, Message> = if let Some((p, _a)) = project_info.clone() {
        row![
            container(Space::new().width(Length::Fixed(6.0)).height(Length::Fixed(6.0)))
                .width(Length::Fixed(6.0))
                .height(Length::Fixed(6.0))
                .style(styles::container::proj_dot(tokens)),
            text(p.name).size(11.5).color(tokens.ink_3),
        ]
        .spacing(5)
        .align_y(Alignment::Center)
        .into()
    } else {
        text("No project").size(13).color(tokens.ink_4).into()
    };

    let mut tags_row = row![].spacing(6).align_y(Alignment::Center);
    for tg in &task.tags {
        tags_row = tags_row.push(
            container(text(tg.clone()).size(11).color(tokens.ink_2))
                .padding(Padding::from([1.0, 7.0]))
                .style(styles::container::tag_pill(tokens)),
        );
    }
    tags_row = tags_row.push(
        button(
            row![
                icons::colored(icons::plus(), 10, tokens.ink_3),
                text("Tag").size(11).color(tokens.ink_3),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([0.0, 8.0]))
        .height(Length::Fixed(22.0))
        .style(styles::button::chip(tokens, false))
        .on_press(Message::Noop),
    );

    let fields = column![
        field_row(tokens, "When", when_btn.into()),
        field_row(tokens, "Deadline", deadline_btn.into()),
        field_row(tokens, "Repeat", repeat_value),
        field_row(tokens, "Project", project_value),
        field_row(tokens, "Tags", tags_row.into()),
    ]
    .spacing(0);

    // Notes
    let notes_input = text_input("Add notes…", &app.inspector_notes_buf)
        .on_input(Message::EditTaskNotes)
        .on_submit(Message::CommitTaskNotes)
        .padding(12)
        .size(13)
        .style(styles::text_input::notes_box(tokens));

    let notes_section = column![
        section_title(tokens, "Notes"),
        notes_input,
    ]
    .spacing(8);

    // Checklist
    let mut checklist_col = column![].spacing(4);
    for c in &task.checklist {
        let cid = c.id.clone();
        let tid = task.id.clone();
        let done_color = if c.done { tokens.ink_4 } else { tokens.ink };
        let mark: Element<'_, Message> = if c.done {
            icons::colored(icons::check(), 10, tokens.on_accent)
        } else {
            Space::new().width(Length::Fixed(10.0)).height(Length::Fixed(10.0)).into()
        };
        let item = button(
            row![
                button(
                    container(mark)
                        .width(Length::Fixed(14.0))
                        .height(Length::Fixed(14.0))
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                )
                .padding(0)
                .width(Length::Fixed(14.0))
                .height(Length::Fixed(14.0))
                .style(styles::button::checkbox_btn(tokens, c.done, false))
                .on_press(Message::ToggleChecklistItem(tid.clone(), cid.clone())),
                text(c.text.clone()).size(13).color(done_color),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([5.0, 4.0]))
        .width(Length::Fill)
        .style(styles::button::small_ghost(tokens))
        .on_press(Message::ToggleChecklistItem(task.id.clone(), c.id.clone()));
        checklist_col = checklist_col.push(item);
    }
    let checklist_add = row![
        container(Space::new().width(Length::Fixed(14.0)).height(Length::Fixed(14.0)))
            .width(Length::Fixed(14.0))
            .height(Length::Fixed(14.0)),
        text_input("Add subtask…", &app.inspector_checklist_buf)
            .on_input(Message::ChecklistAddChanged)
            .on_submit(Message::ChecklistAddSubmit)
            .padding(0)
            .size(13)
            .style(styles::text_input::flat(tokens)),
    ]
    .spacing(10)
    .align_y(Alignment::Center);
    let checklist_add_box = container(checklist_add).padding(Padding::from([5.0, 4.0]));
    checklist_col = checklist_col.push(checklist_add_box);

    let checklist_section = column![section_title(tokens, "Checklist"), checklist_col].spacing(8);

    // Activity
    let activity = column![
        section_title(tokens, "Activity"),
        text("Created · 3 days ago").size(12).color(tokens.ink_4),
        text(format!(
            "Moved to {} · 2 days ago",
            project_info.as_ref().map(|(p, _)| p.name.clone()).unwrap_or_else(|| "Inbox".into())
        ))
        .size(12)
        .color(tokens.ink_4),
    ]
    .spacing(2);

    let body = column![fields, notes_section, checklist_section, activity].spacing(18);

    let body_box = container(body).padding(Padding {
        top: 14.0,
        right: 20.0,
        bottom: 24.0,
        left: 20.0,
    });

    let scroll = scrollable(body_box)
        .style(styles::scrollable::calm(tokens))
        .height(Length::Fill)
        .width(Length::Fill);

    let pane = container(column![header, header_border, scroll])
        .width(Length::Fixed(320.0))
        .height(Length::Fill)
        .style(styles::container::inspector(tokens));

    let left_border = container(Space::new().width(Length::Fixed(1.0)))
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(styles::container::divider(tokens));

    row![left_border, pane].into()
}

fn field_row<'a>(
    tokens: crate::theme::Tokens,
    label: &'a str,
    value: Element<'a, Message>,
) -> Element<'a, Message> {
    container(
        row![
            container(text(label).size(11.5).color(tokens.ink_4))
                .width(Length::Fixed(88.0)),
            container(value).width(Length::Fill),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([7.0, 0.0]))
    .into()
}

fn section_title<'a>(
    tokens: crate::theme::Tokens,
    label: &'a str,
) -> Element<'a, Message> {
    container(text(label.to_uppercase()).size(11).color(tokens.ink_4))
        .padding(Padding {
            top: 18.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .into()
}
