use iced::widget::{Space, button, container, mouse_area, row, text};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{App, Message};
use crate::date::format_date;
use crate::models::{DateField, DateTarget, DueState, Task};
use crate::styles;
use crate::ui::icons;

pub fn view<'a>(app: &'a App, task: &'a Task) -> Element<'a, Message> {
    let tokens = app.tokens;
    let selected = app.selection.contains(&task.id);
    let project_name = task
        .project
        .as_ref()
        .and_then(|pid| {
            app.areas
                .iter()
                .flat_map(|a| &a.projects)
                .find(|p| &p.id == pid)
        })
        .map(|p| p.name.clone());

    // Grip
    let grip_visual = container(icons::colored(icons::grip(), 10, tokens.ink_4))
        .width(Length::Fixed(24.0))
        .height(Length::Fixed(app.density.row_h() as f32))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    let grip = mouse_area(grip_visual)
        .interaction(iced::mouse::Interaction::Grab)
        .on_press(Message::DragBegin(task.id.clone()));

    // Checkbox
    let cb_inner: Element<'a, Message> = if task.done {
        icons::colored(icons::check(), 12, tokens.on_accent)
    } else {
        Space::new()
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0))
            .into()
    };
    let checkbox = button(
        container(cb_inner)
            .width(Length::Fixed(18.0))
            .height(Length::Fixed(18.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(0)
    .width(Length::Fixed(18.0))
    .height(Length::Fixed(18.0))
    .style(styles::button::checkbox_btn(
        tokens,
        task.done,
        task.deadline.is_some(),
    ))
    .on_press(Message::CheckTask(task.id.clone()));

    // Burst overlay if active
    let cb_with_burst: Element<'a, Message> = if let Some(start) = app.bursting.get(&task.id) {
        let elapsed = start.elapsed().as_millis() as f32;
        let progress = (elapsed / 500.0).clamp(0.0, 1.0);
        let burst = crate::ui::checkbox::burst_overlay::<Message>(progress, tokens.accent, 22.0);
        let centered_cb = container(checkbox)
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);
        let burst_box = container(burst)
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0));
        iced::widget::stack![centered_cb, burst_box].into()
    } else {
        checkbox.into()
    };

    let cb_cell = container(cb_with_burst)
        .width(Length::Fixed(22.0))
        .height(Length::Fixed(app.density.row_h() as f32))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

    // Title cluster
    let title_color = if task.done { tokens.ink_4 } else { tokens.ink };
    let title_text = text(task.title.clone())
        .size(14)
        .color(title_color)
        .width(Length::Shrink);
    let mut title_row = row![title_text].spacing(8).align_y(Alignment::Center);
    if !task.notes.trim().is_empty() {
        title_row = title_row.push(
            container(
                Space::new()
                    .width(Length::Fixed(4.0))
                    .height(Length::Fixed(4.0)),
            )
            .width(Length::Fixed(4.0))
            .height(Length::Fixed(4.0))
            .style(notes_dot(tokens)),
        );
    }
    if !task.checklist.is_empty() {
        let done_n = task.checklist.iter().filter(|c| c.done).count();
        let total_n = task.checklist.len();
        let chk_pill = container(
            text(format!("\u{2713} {}/{}", done_n, total_n))
                .size(11)
                .color(tokens.ink_3),
        )
        .padding(Padding::from([1.0, 6.0]))
        .style(styles::container::checklist_mini(tokens));
        title_row = title_row.push(chk_pill);
    }

    let title_cell = container(title_row).width(Length::Fill);

    // Meta cluster
    let mut meta_children: Vec<Element<'a, Message>> = Vec::new();

    if task.repeat.is_some() {
        meta_children.push(icons::colored(icons::repeat(), 12, tokens.ink_4));
    }
    if let Some(tag) = task.tags.first() {
        meta_children.push(
            container(text(tag.clone()).size(11).color(tokens.ink_2))
                .padding(Padding::from([1.0, 7.0]))
                .style(styles::container::tag_pill(tokens))
                .into(),
        );
    }
    if let Some(name) = project_name.as_ref() {
        let dot = container(
            Space::new()
                .width(Length::Fixed(6.0))
                .height(Length::Fixed(6.0)),
        )
        .width(Length::Fixed(6.0))
        .height(Length::Fixed(6.0))
        .style(styles::container::proj_dot(tokens));
        meta_children.push(
            row![dot, text(name.clone()).size(11.5).color(tokens.ink_3)]
                .spacing(5)
                .align_y(Alignment::Center)
                .into(),
        );
    }
    if let Some(when) = task.when {
        if !task.done {
            let label = format_date(when, app.today);
            let state = crate::date::due_state(when, app.today);
            let color = match state {
                DueState::Today => {
                    if tokens.is_dark {
                        tokens.accent
                    } else {
                        tokens.accent_ink
                    }
                }
                DueState::Overdue => tokens.danger,
                DueState::Scheduled => tokens.ink_3,
            };
            let due_chip = mouse_area(
                row![
                    icons::colored(icons::calendar(), 12, color),
                    text(label).size(12).color(color),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .on_press(Message::OpenDatePicker {
                target: DateTarget::Single(task.id.clone()),
                field: DateField::When,
                anchor: crate::models::AnchorKind::MainCenter,
                initial: Some(when),
            });
            meta_children.push(due_chip.into());
        }
    } else if task.done
        && let Some(da) = task.done_at
    {
        meta_children.push(
            text(format_date(da, app.today))
                .size(12)
                .color(tokens.ink_3)
                .into(),
        );
    }

    let meta_cell = container(row(meta_children).spacing(10).align_y(Alignment::Center));

    // Compose
    let row_inner = row![grip, cb_cell, title_cell, meta_cell]
        .spacing(10)
        .align_y(Alignment::Center)
        .height(Length::Fixed(app.density.row_h() as f32));

    let is_drop_target = app
        .dragging
        .as_ref()
        .and_then(|d| d.drop_target.as_ref())
        .map(|t| t == &task.id)
        .unwrap_or(false);

    let row_btn = button(row_inner)
        .padding(Padding::from([app.density.pad_y() as f32, 8.0]))
        .width(Length::Fill)
        .style(styles::button::task_row(tokens, selected, false))
        .on_press(Message::OpenTaskWithMods(task.id.clone()));

    let row_with_hover = mouse_area(row_btn).on_enter(Message::DragHover(task.id.clone()));

    if is_drop_target {
        let indicator = container(Space::new().height(Length::Fixed(2.0)))
            .width(Length::Fill)
            .height(Length::Fixed(2.0))
            .style(styles::container::drop_indicator(tokens));
        iced::widget::column![indicator, Element::from(row_with_hover)].into()
    } else {
        row_with_hover.into()
    }
}

fn notes_dot(
    tokens: crate::theme::Tokens,
) -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |_theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(tokens.ink_4)),
        border: iced::Border {
            color: tokens.ink_4,
            width: 0.0,
            radius: 999.0.into(),
        },
        text_color: None,
        shadow: Default::default(),
        ..Default::default()
    }
}
