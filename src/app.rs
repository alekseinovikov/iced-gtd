use std::collections::{HashMap, HashSet};
use std::time::Instant;

use chrono::{Datelike, Local, NaiveDate};
use iced::keyboard::{self, Modifiers};
use iced::{Element, Event, Subscription, Task, Theme, event, window};

use crate::data;
use crate::date::format_date;
use crate::models::{
    AnchorKind, Area, AreaId, ChecklistId, DateField, DatePickerState, DateTarget, Density,
    DueState, Filter, GroupKey, Project, ProjectId, Task as TaskModel, TaskId, ThemeMode, View,
};
use crate::theme::Tokens;
use crate::ui;

pub struct DragState {
    pub task_id: TaskId,
    pub start_y: f32,
    pub current_y: f32,
    pub drop_target: Option<TaskId>,
}

pub struct App {
    pub tasks: Vec<TaskModel>,
    pub areas: Vec<Area>,
    pub view: View,
    pub active_project: Option<ProjectId>,
    pub open_task: Option<TaskId>,
    pub selection: HashSet<TaskId>,
    pub show_sidebar: bool,
    pub show_inspector: bool,
    pub date_picker: Option<DatePickerState>,
    pub theme_mode: ThemeMode,
    pub density: Density,
    pub tokens: Tokens,
    pub collapsed_areas: HashSet<AreaId>,
    pub composer_buffers: HashMap<String, String>,
    pub composer_focused: Option<String>,
    pub modifiers: Modifiers,
    pub bursting: HashMap<TaskId, Instant>,
    pub dragging: Option<DragState>,
    pub filter: Filter,
    pub today: NaiveDate,
    pub search_query: String,
    pub sidebar_open: f32,                   // 0.0..=1.0
    pub chevron_anims: HashMap<AreaId, f32>, // 0..1, 1 = collapsed
    pub inspector_title_buf: String,
    pub inspector_notes_buf: String,
    pub inspector_checklist_buf: String,
    pub inspector_open_id: Option<TaskId>,
    pub window_id: Option<window::Id>,
    pub initial_data_loaded: bool,
    pub menu_open: bool, // density/theme tweaks popover
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectView(View),
    SelectProject(ProjectId),
    ToggleAreaCollapse(AreaId),
    OpenTask(TaskId),
    OpenTaskWithMods(TaskId),
    CheckTask(TaskId),
    AddTaskAtView,
    AddTaskInGroup(String, String),
    EditTaskTitle(String),
    CommitTaskTitle,
    EditTaskNotes(String),
    CommitTaskNotes,
    ScheduleTaskField(TaskId, DateField, Option<NaiveDate>),
    ToggleChecklistItem(TaskId, ChecklistId),
    ChecklistAddChanged(String),
    ChecklistAddSubmit,
    ReorderTask(TaskId, TaskId),
    BulkComplete,
    BulkScheduleSelected,
    BulkDelete,
    ClearSelection,
    OpenDatePicker {
        target: DateTarget,
        field: DateField,
        anchor: AnchorKind,
        initial: Option<NaiveDate>,
    },
    CloseDatePicker,
    DatePickerNav(i32),
    DatePickerPick(Option<NaiveDate>),
    ToggleSidebar,
    ToggleInspector,
    SetTheme(ThemeMode),
    SetDensity(Density),
    SidebarSearchChanged(String),
    ChipFilter(Filter),
    ComposerChanged(String, String),
    ComposerFocus(Option<String>),
    ComposerSubmit(String),
    ToggleSelectClicked(TaskId),
    InspectorClose,
    DeselectAll,
    ModifiersChanged(Modifiers),
    KeyPressed(keyboard::Key, Modifiers),
    Tick(Instant),
    DragWindow,
    WindowOpened(window::Id),
    WindowClose,
    WindowMinimize,
    OpenMenu,
    CloseMenu,
    DragBegin(TaskId),
    DragHover(TaskId),
    DragEnd,
    Loaded(Result<crate::storage::Store, String>),
    Saved(Result<(), String>),
    Noop,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let today = Local::now().date_naive();
        let tasks = data::make_initial_tasks(today);
        let areas = data::initial_areas();
        let app = App {
            tasks,
            areas,
            view: View::Today,
            active_project: None,
            open_task: Some("t1".into()),
            selection: HashSet::new(),
            show_sidebar: true,
            show_inspector: true,
            date_picker: None,
            theme_mode: ThemeMode::Light,
            density: Density::Cozy,
            tokens: Tokens::light(),
            collapsed_areas: HashSet::new(),
            composer_buffers: HashMap::new(),
            composer_focused: None,
            modifiers: Modifiers::default(),
            bursting: HashMap::new(),
            dragging: None,
            filter: Filter::All,
            today,
            search_query: String::new(),
            sidebar_open: 1.0,
            chevron_anims: HashMap::new(),
            inspector_title_buf: String::new(),
            inspector_notes_buf: String::new(),
            inspector_checklist_buf: String::new(),
            inspector_open_id: None,
            window_id: None,
            initial_data_loaded: false,
            menu_open: false,
        };
        (app, Task::perform(crate::storage::load(), Message::Loaded))
    }

    fn snapshot(&self) -> crate::storage::Store {
        crate::storage::Store {
            schema_version: 1,
            tasks: self.tasks.clone(),
            areas: self.areas.clone(),
            theme_mode: self.theme_mode,
            density: self.density,
            collapsed_areas: self.collapsed_areas.iter().cloned().collect(),
        }
    }

    fn persist(&self) -> Task<Message> {
        if !self.initial_data_loaded {
            return Task::none();
        }
        Task::perform(crate::storage::save(self.snapshot()), Message::Saved)
    }

    pub fn theme(&self) -> Theme {
        if matches!(self.theme_mode, ThemeMode::Dark) {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard::Event::ModifiersChanged(mods)) => {
                Some(Message::ModifiersChanged(mods))
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                Some(Message::KeyPressed(key, modifiers))
            }
            Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                Some(Message::DragEnd)
            }
            _ => None,
        })];
        if self.window_id.is_none() {
            subs.push(window::open_events().map(Message::WindowOpened));
        }
        if !self.bursting.is_empty() {
            subs.push(iced::time::every(std::time::Duration::from_millis(16)).map(Message::Tick));
        }
        Subscription::batch(subs)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectView(v) => {
                self.view = v;
                self.active_project = None;
                self.selection.clear();
                Task::none()
            }
            Message::SelectProject(id) => {
                self.active_project = Some(id);
                self.view = View::Project;
                self.selection.clear();
                Task::none()
            }
            Message::ToggleAreaCollapse(id) => {
                if self.collapsed_areas.contains(&id) {
                    self.collapsed_areas.remove(&id);
                } else {
                    self.collapsed_areas.insert(id);
                }
                self.persist()
            }
            Message::OpenTask(id) => {
                self.open_task = Some(id.clone());
                self.show_inspector = true;
                self.sync_inspector_buffers();
                Task::none()
            }
            Message::OpenTaskWithMods(id) => {
                if self.modifiers.command() || self.modifiers.control() {
                    if self.selection.contains(&id) {
                        self.selection.remove(&id);
                    } else {
                        self.selection.insert(id);
                    }
                } else {
                    self.open_task = Some(id);
                    self.show_inspector = true;
                    self.sync_inspector_buffers();
                }
                Task::none()
            }
            Message::ToggleSelectClicked(id) => {
                if self.selection.contains(&id) {
                    self.selection.remove(&id);
                } else {
                    self.selection.insert(id);
                }
                Task::none()
            }
            Message::CheckTask(id) => {
                let mut becoming_done = false;
                if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                    t.done = !t.done;
                    if t.done {
                        t.done_at = Some(self.today);
                        becoming_done = true;
                    } else {
                        t.done_at = None;
                    }
                }
                if becoming_done {
                    self.bursting.insert(id, Instant::now());
                }
                self.persist()
            }
            Message::AddTaskAtView => {
                let group = match self.view {
                    View::Today => "today".to_string(),
                    View::Inbox => "inbox".to_string(),
                    View::Anytime => "anytime".to_string(),
                    View::Someday => "someday".to_string(),
                    View::Upcoming => "upcoming".to_string(),
                    View::Project => "open".to_string(),
                    _ => "today".to_string(),
                };
                self.add_task(group, "New task".to_string());
                self.persist()
            }
            Message::AddTaskInGroup(group_key, title) => {
                self.add_task(group_key, title);
                self.persist()
            }
            Message::ComposerChanged(key, val) => {
                self.composer_buffers.insert(key, val);
                Task::none()
            }
            Message::ComposerFocus(key) => {
                self.composer_focused = key;
                Task::none()
            }
            Message::ComposerSubmit(key) => {
                if let Some(buf) = self.composer_buffers.get(&key).cloned() {
                    let trimmed = buf.trim();
                    if !trimmed.is_empty() {
                        self.add_task(key.clone(), trimmed.to_string());
                        self.composer_buffers.insert(key, String::new());
                        return self.persist();
                    }
                }
                Task::none()
            }
            Message::EditTaskTitle(s) => {
                self.inspector_title_buf = s;
                Task::none()
            }
            Message::CommitTaskTitle => {
                if let Some(id) = self.open_task.clone() {
                    let new = if self.inspector_title_buf.trim().is_empty() {
                        "Untitled".to_string()
                    } else {
                        self.inspector_title_buf.clone()
                    };
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        t.title = new;
                    }
                }
                self.persist()
            }
            Message::EditTaskNotes(s) => {
                self.inspector_notes_buf = s;
                Task::none()
            }
            Message::CommitTaskNotes => {
                if let Some(id) = self.open_task.clone() {
                    let val = self.inspector_notes_buf.clone();
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        t.notes = val;
                    }
                }
                self.persist()
            }
            Message::ScheduleTaskField(id, field, value) => {
                if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                    match field {
                        DateField::When => t.when = value,
                        DateField::Deadline => t.deadline = value,
                    }
                }
                self.persist()
            }
            Message::ToggleChecklistItem(id, cid) => {
                if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                    for c in t.checklist.iter_mut() {
                        if c.id == cid {
                            c.done = !c.done;
                        }
                    }
                }
                self.persist()
            }
            Message::ChecklistAddChanged(s) => {
                self.inspector_checklist_buf = s;
                Task::none()
            }
            Message::ChecklistAddSubmit => {
                if let Some(id) = self.open_task.clone() {
                    let trimmed = self.inspector_checklist_buf.trim().to_string();
                    if !trimmed.is_empty() {
                        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                            let cid = format!("c{}", chrono::Utc::now().timestamp_millis());
                            t.checklist.push(crate::models::ChecklistItem {
                                id: cid,
                                text: trimmed,
                                done: false,
                            });
                        }
                        self.inspector_checklist_buf.clear();
                    }
                }
                self.persist()
            }
            Message::ReorderTask(from, to) => {
                let from_idx = self.tasks.iter().position(|t| t.id == from);
                let to_idx = self.tasks.iter().position(|t| t.id == to);
                if let (Some(f), Some(t)) = (from_idx, to_idx) {
                    let item = self.tasks.remove(f);
                    self.tasks.insert(t, item);
                }
                self.persist()
            }
            Message::BulkComplete => {
                let ids: Vec<_> = self.selection.iter().cloned().collect();
                for id in ids {
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        t.done = true;
                        t.done_at = Some(self.today);
                    }
                }
                self.selection.clear();
                self.persist()
            }
            Message::BulkScheduleSelected => {
                self.date_picker = Some(DatePickerState {
                    target: DateTarget::Bulk,
                    field: DateField::When,
                    anchor: AnchorKind::BulkBar,
                    view_year: self.today.year(),
                    view_month: self.today.month(),
                    initial_value: None,
                });
                Task::none()
            }
            Message::BulkDelete => {
                let to_delete = self.selection.clone();
                self.tasks.retain(|t| !to_delete.contains(&t.id));
                self.selection.clear();
                self.persist()
            }
            Message::ClearSelection => {
                self.selection.clear();
                Task::none()
            }
            Message::DeselectAll => {
                self.selection.clear();
                Task::none()
            }
            Message::OpenDatePicker {
                target,
                field,
                anchor,
                initial,
            } => {
                let (y, m) = match initial {
                    Some(d) => (d.year(), d.month()),
                    None => (self.today.year(), self.today.month()),
                };
                self.date_picker = Some(DatePickerState {
                    target,
                    field,
                    anchor,
                    view_year: y,
                    view_month: m,
                    initial_value: initial,
                });
                Task::none()
            }
            Message::CloseDatePicker => {
                self.date_picker = None;
                Task::none()
            }
            Message::DatePickerNav(delta) => {
                if let Some(dp) = self.date_picker.as_mut() {
                    let mut m = dp.view_month as i32 + delta;
                    let mut y = dp.view_year;
                    while m <= 0 {
                        m += 12;
                        y -= 1;
                    }
                    while m > 12 {
                        m -= 12;
                        y += 1;
                    }
                    dp.view_year = y;
                    dp.view_month = m as u32;
                }
                Task::none()
            }
            Message::DatePickerPick(date) => {
                if let Some(dp) = self.date_picker.clone() {
                    match dp.target {
                        DateTarget::Single(id) => {
                            if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                                match dp.field {
                                    DateField::When => t.when = date,
                                    DateField::Deadline => t.deadline = date,
                                }
                            }
                        }
                        DateTarget::Bulk => {
                            for id in self.selection.iter() {
                                if let Some(t) = self.tasks.iter_mut().find(|t| &t.id == id) {
                                    t.when = date;
                                }
                            }
                        }
                    }
                }
                self.date_picker = None;
                self.persist()
            }
            Message::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
                Task::none()
            }
            Message::ToggleInspector => {
                self.show_inspector = !self.show_inspector;
                Task::none()
            }
            Message::InspectorClose => {
                self.show_inspector = false;
                Task::none()
            }
            Message::SetTheme(mode) => {
                self.theme_mode = mode;
                self.tokens = match mode {
                    ThemeMode::Light => Tokens::light(),
                    ThemeMode::Dark => Tokens::dark(),
                };
                self.persist()
            }
            Message::SetDensity(d) => {
                self.density = d;
                self.persist()
            }
            Message::SidebarSearchChanged(s) => {
                self.search_query = s;
                Task::none()
            }
            Message::ChipFilter(f) => {
                self.filter = f;
                Task::none()
            }
            Message::ModifiersChanged(m) => {
                self.modifiers = m;
                Task::none()
            }
            Message::KeyPressed(key, mods) => {
                match key {
                    keyboard::Key::Named(keyboard::key::Named::Escape) => {
                        if self.date_picker.is_some() {
                            self.date_picker = None;
                        } else if !self.selection.is_empty() {
                            self.selection.clear();
                        } else {
                            self.composer_focused = None;
                        }
                    }
                    keyboard::Key::Character(ref c) if (mods.command() || mods.control()) => {
                        let lower = c.to_lowercase();
                        if lower == "n" {
                            return self.update(Message::AddTaskAtView);
                        } else if lower == "k" {
                            return iced::widget::operation::focus(iced::widget::Id::new(
                                "sidebar-search",
                            ));
                        }
                    }
                    _ => {}
                }
                Task::none()
            }
            Message::Tick(_) => {
                let cutoff = Instant::now();
                self.bursting
                    .retain(|_, t| cutoff.duration_since(*t).as_millis() < 500);
                Task::none()
            }
            Message::DragWindow => {
                if let Some(id) = self.window_id {
                    return window::drag(id);
                }
                Task::none()
            }
            Message::WindowOpened(id) => {
                self.window_id = Some(id);
                Task::none()
            }
            Message::WindowClose => {
                if let Some(id) = self.window_id {
                    return window::close(id);
                }
                Task::none()
            }
            Message::WindowMinimize => {
                if let Some(id) = self.window_id {
                    return window::minimize(id, true);
                }
                Task::none()
            }
            Message::OpenMenu => {
                self.menu_open = true;
                Task::none()
            }
            Message::CloseMenu => {
                self.menu_open = false;
                Task::none()
            }
            Message::DragBegin(id) => {
                self.dragging = Some(DragState {
                    task_id: id,
                    start_y: 0.0,
                    current_y: 0.0,
                    drop_target: None,
                });
                Task::none()
            }
            Message::DragHover(id) => {
                if let Some(d) = self.dragging.as_mut() {
                    if d.task_id != id {
                        d.drop_target = Some(id);
                    } else {
                        d.drop_target = None;
                    }
                }
                Task::none()
            }
            Message::DragEnd => {
                if let Some(d) = self.dragging.take()
                    && let Some(target) = d.drop_target
                {
                    let from_idx = self.tasks.iter().position(|t| t.id == d.task_id);
                    let to_idx = self.tasks.iter().position(|t| t.id == target);
                    if let (Some(f), Some(to)) = (from_idx, to_idx) {
                        let item = self.tasks.remove(f);
                        // After removing at f, indices > f have shifted left by 1.
                        let insert_at = if f < to { to - 1 } else { to };
                        self.tasks.insert(insert_at.min(self.tasks.len()), item);
                        return self.persist();
                    }
                }
                Task::none()
            }
            Message::Loaded(Ok(store)) => {
                self.tasks = store.tasks;
                self.areas = store.areas;
                self.theme_mode = store.theme_mode;
                self.tokens = match self.theme_mode {
                    ThemeMode::Light => Tokens::light(),
                    ThemeMode::Dark => Tokens::dark(),
                };
                self.density = store.density;
                self.collapsed_areas = store.collapsed_areas.into_iter().collect();
                self.initial_data_loaded = true;
                if let Some(id) = self.open_task.clone()
                    && !self.tasks.iter().any(|t| t.id == id)
                {
                    self.open_task = self.tasks.first().map(|t| t.id.clone());
                }
                self.sync_inspector_buffers();
                Task::none()
            }
            Message::Loaded(Err(_)) => {
                self.initial_data_loaded = true;
                self.persist()
            }
            Message::Saved(Ok(())) => Task::none(),
            Message::Saved(Err(e)) => {
                eprintln!("save failed: {}", e);
                Task::none()
            }
            Message::Noop => Task::none(),
        }
    }

    fn add_task(&mut self, group_key: String, title: String) {
        let when = if self.view == View::Today
            || self.view == View::Upcoming
            || group_key == "today"
            || group_key == "evening"
        {
            Some(self.today)
        } else {
            None
        };
        let evening = group_key == "evening";
        let project = match self.view {
            View::Project => self.active_project.clone(),
            _ => None,
        };
        let id = format!("t{}", chrono::Utc::now().timestamp_millis());
        let new_task = TaskModel {
            id: id.clone(),
            title,
            when,
            deadline: None,
            project,
            tags: Vec::new(),
            notes: String::new(),
            checklist: Vec::new(),
            done: false,
            done_at: None,
            evening,
            repeat: None,
        };
        self.tasks.push(new_task);
        self.open_task = Some(id);
        self.show_inspector = true;
        self.sync_inspector_buffers();
    }

    pub fn sync_inspector_buffers(&mut self) {
        if let Some(id) = self.open_task.clone() {
            if Some(id.clone()) != self.inspector_open_id
                && let Some(t) = self.tasks.iter().find(|t| t.id == id)
            {
                self.inspector_title_buf = t.title.clone();
                self.inspector_notes_buf = t.notes.clone();
                self.inspector_checklist_buf.clear();
                self.inspector_open_id = Some(id);
            }
        } else {
            self.inspector_open_id = None;
        }
    }

    pub fn projects_by_id(&self) -> HashMap<ProjectId, (Project, AreaId, String)> {
        let mut map = HashMap::new();
        for area in &self.areas {
            for p in &area.projects {
                map.insert(
                    p.id.clone(),
                    (p.clone(), area.id.clone(), area.name.clone()),
                );
            }
        }
        map
    }

    pub fn area_by_id(&self, id: &AreaId) -> Option<&Area> {
        self.areas.iter().find(|a| &a.id == id)
    }

    pub fn open_task_ref(&self) -> Option<&TaskModel> {
        self.open_task
            .as_ref()
            .and_then(|id| self.tasks.iter().find(|t| &t.id == id))
    }

    pub fn counts(&self) -> Counts {
        let mut inbox = 0;
        let mut today_n = 0;
        let mut upcoming = 0;
        let mut anytime = 0;
        for t in &self.tasks {
            if t.done {
                continue;
            }
            if t.project.is_none() && t.when.is_none() {
                inbox += 1;
            }
            if let Some(when) = t.when {
                if when <= self.today {
                    today_n += 1;
                } else {
                    upcoming += 1;
                }
            } else {
                anytime += 1;
            }
        }
        Counts {
            inbox,
            today: today_n,
            upcoming,
            anytime,
        }
    }

    pub fn header_info(&self) -> HeaderInfo {
        match self.view {
            View::Project => {
                if let Some(pid) = self.active_project.as_ref() {
                    let map = self.projects_by_id();
                    if let Some((p, _aid, area_name)) = map.get(pid) {
                        let ts: Vec<&TaskModel> = self
                            .tasks
                            .iter()
                            .filter(|t| t.project.as_deref() == Some(pid))
                            .collect();
                        let done = ts.iter().filter(|t| t.done).count();
                        return HeaderInfo {
                            title: p.name.clone(),
                            meta: format!("{}/{} done", done, ts.len()),
                            sub: Some(format!("{} · {} open", area_name, ts.len() - done)),
                        };
                    }
                }
                HeaderInfo::default()
            }
            View::Inbox => HeaderInfo {
                title: "Inbox".into(),
                meta: String::new(),
                sub: Some("Capture freely. Sort later.".into()),
            },
            View::Today => HeaderInfo {
                title: "Today".into(),
                meta: crate::date::long_header_label(self.today),
                sub: None,
            },
            View::Upcoming => HeaderInfo {
                title: "Upcoming".into(),
                meta: "Next 30 days".into(),
                sub: None,
            },
            View::Anytime => HeaderInfo {
                title: "Anytime".into(),
                meta: String::new(),
                sub: Some("Tasks without a date — pick when you have a moment.".into()),
            },
            View::Someday => HeaderInfo {
                title: "Someday".into(),
                meta: String::new(),
                sub: Some("Maybe later. Maybe never.".into()),
            },
            View::Logbook => HeaderInfo {
                title: "Logbook".into(),
                meta: "All-time".into(),
                sub: None,
            },
            View::Trash => HeaderInfo {
                title: "Trash".into(),
                meta: String::new(),
                sub: None,
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        use iced::Length;
        use iced::widget::{column, container, row, stack};

        let titlebar = ui::titlebar::view(self);
        let mut panes_row = row![].height(Length::Fill);
        if self.show_sidebar {
            panes_row = panes_row.push(ui::sidebar::view(self));
        }
        panes_row = panes_row.push(
            container(ui::main_pane::view(self))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(crate::styles::container::main_pane(self.tokens)),
        );
        if self.show_inspector && self.open_task.is_some() {
            panes_row = panes_row.push(ui::inspector::view(self));
        }

        let body = column![titlebar, panes_row].height(Length::Fill);

        let shell = container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::styles::container::window_shell(self.tokens));

        // Overlays: bulk bar + date picker (Phase 2/4 will surface them)
        let mut layers: Vec<Element<'_, Message>> = vec![shell.into()];
        if !self.selection.is_empty() {
            layers.push(ui::main_pane::bulk_bar_overlay(self));
        }
        if self.date_picker.is_some() {
            layers.push(ui::date_picker::view(self));
        }
        if layers.len() == 1 {
            layers.pop().unwrap()
        } else {
            let mut s = stack![];
            for l in layers {
                s = s.push(l);
            }
            s.into()
        }
    }

    pub fn render_due_label(&self, when: NaiveDate) -> (String, DueState) {
        (
            format_date(when, self.today),
            crate::date::due_state(when, self.today),
        )
    }
}

#[derive(Default)]
pub struct HeaderInfo {
    pub title: String,
    pub meta: String,
    pub sub: Option<String>,
}

#[derive(Default, Debug)]
pub struct Counts {
    pub inbox: u32,
    pub today: u32,
    pub upcoming: u32,
    pub anytime: u32,
}

#[derive(Debug, Clone)]
pub struct GroupedTasks {
    pub key: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub tasks: Vec<TaskId>,
    pub allow_add: bool,
    pub add_placeholder: String,
    pub empty_hint: Option<String>,
}

pub fn build_groups(app: &App) -> Vec<GroupedTasks> {
    let today = app.today;
    let filter = app.filter;
    let in_filter = |t: &TaskModel| filter.matches(&t.tags);
    let visible: Vec<&TaskModel> = app.tasks.iter().filter(|t| in_filter(t)).collect();

    match app.view {
        View::Project => {
            if let Some(pid) = app.active_project.as_ref() {
                let scoped: Vec<&TaskModel> = visible
                    .iter()
                    .filter(|t| t.project.as_deref() == Some(pid))
                    .copied()
                    .collect();
                let open: Vec<TaskId> = scoped
                    .iter()
                    .filter(|t| !t.done)
                    .map(|t| t.id.clone())
                    .collect();
                let done: Vec<TaskId> = scoped
                    .iter()
                    .filter(|t| t.done)
                    .map(|t| t.id.clone())
                    .collect();
                let mut groups = vec![GroupedTasks {
                    key: "open".into(),
                    title: String::new(),
                    subtitle: None,
                    tasks: open,
                    allow_add: true,
                    add_placeholder: "Add to project…".into(),
                    empty_hint: None,
                }];
                if !done.is_empty() {
                    let n = done.len();
                    groups.push(GroupedTasks {
                        key: "done".into(),
                        title: "Completed".into(),
                        subtitle: Some(format!("{}", n)),
                        tasks: done,
                        allow_add: false,
                        add_placeholder: String::new(),
                        empty_hint: None,
                    });
                }
                groups
            } else {
                Vec::new()
            }
        }
        View::Inbox => {
            let inbox: Vec<TaskId> = visible
                .iter()
                .filter(|t| !t.done && t.project.is_none() && t.when.is_none())
                .map(|t| t.id.clone())
                .collect();
            vec![GroupedTasks {
                key: "inbox".into(),
                title: String::new(),
                subtitle: None,
                tasks: inbox,
                allow_add: true,
                add_placeholder: "Capture anything on your mind…".into(),
                empty_hint: Some("Inbox zero. Quick-capture with ⌘⇧Space.".into()),
            }]
        }
        View::Today => {
            let mut overdue = Vec::new();
            let mut morning = Vec::new();
            let mut evening = Vec::new();
            for t in &visible {
                if t.done {
                    continue;
                }
                if let Some(when) = t.when {
                    if when < today {
                        overdue.push(t.id.clone());
                    } else if when == today {
                        if t.evening {
                            evening.push(t.id.clone());
                        } else {
                            morning.push(t.id.clone());
                        }
                    }
                }
            }
            let mut groups = Vec::new();
            if !overdue.is_empty() {
                let n = overdue.len();
                groups.push(GroupedTasks {
                    key: "overdue".into(),
                    title: "Overdue".into(),
                    subtitle: Some(format!("{}", n)),
                    tasks: overdue,
                    allow_add: false,
                    add_placeholder: String::new(),
                    empty_hint: None,
                });
            }
            groups.push(GroupedTasks {
                key: "today".into(),
                title: String::new(),
                subtitle: None,
                tasks: morning,
                allow_add: true,
                add_placeholder: "What do you want to do today?".into(),
                empty_hint: None,
            });
            groups.push(GroupedTasks {
                key: "evening".into(),
                title: "This evening".into(),
                subtitle: None,
                tasks: evening,
                allow_add: true,
                add_placeholder: "Add for this evening…".into(),
                empty_hint: None,
            });
            groups
        }
        View::Upcoming => {
            let mut buckets: std::collections::BTreeMap<NaiveDate, Vec<TaskId>> =
                Default::default();
            for t in &visible {
                if t.done {
                    continue;
                }
                if let Some(when) = t.when
                    && when > today
                {
                    buckets.entry(when).or_default().push(t.id.clone());
                }
            }
            buckets
                .into_iter()
                .map(|(d, tasks)| GroupedTasks {
                    key: format!("upcoming-{}", d),
                    title: crate::date::weekday_long(d.weekday()).to_string(),
                    subtitle: Some(format!(
                        "{} {}",
                        crate::date::month_short(d.month()),
                        d.day()
                    )),
                    tasks,
                    allow_add: false,
                    add_placeholder: String::new(),
                    empty_hint: None,
                })
                .collect()
        }
        View::Anytime => {
            let v: Vec<TaskId> = visible
                .iter()
                .filter(|t| !t.done && t.when.is_none())
                .map(|t| t.id.clone())
                .collect();
            vec![GroupedTasks {
                key: "anytime".into(),
                title: String::new(),
                subtitle: None,
                tasks: v,
                allow_add: true,
                add_placeholder: "Add a task…".into(),
                empty_hint: None,
            }]
        }
        View::Someday => vec![GroupedTasks {
            key: "someday".into(),
            title: String::new(),
            subtitle: None,
            tasks: Vec::new(),
            allow_add: true,
            add_placeholder: "Park an idea…".into(),
            empty_hint: Some("Park ideas here you might do later.".into()),
        }],
        View::Logbook => {
            let mut v: Vec<&TaskModel> = visible.iter().filter(|t| t.done).copied().collect();
            v.sort_by_key(|t| std::cmp::Reverse(t.done_at));
            let ids = v.iter().map(|t| t.id.clone()).collect();
            vec![GroupedTasks {
                key: "log".into(),
                title: "Recently completed".into(),
                subtitle: None,
                tasks: ids,
                allow_add: false,
                add_placeholder: String::new(),
                empty_hint: None,
            }]
        }
        View::Trash => vec![GroupedTasks {
            key: "trash".into(),
            title: String::new(),
            subtitle: None,
            tasks: Vec::new(),
            allow_add: false,
            add_placeholder: String::new(),
            empty_hint: Some("Trash is empty.".into()),
        }],
    }
}

// Re-export GroupKey alias if needed
pub fn group_key_string(g: &GroupKey) -> &str {
    &g.0
}
