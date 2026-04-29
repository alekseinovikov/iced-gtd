use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type TaskId = String;
pub type AreaId = String;
pub type ProjectId = String;
pub type ChecklistId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: ChecklistId,
    pub text: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectId>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub checklist: Vec<ChecklistItem>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub done: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done_at: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub evening: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    #[serde(default)]
    pub emoji: String,
    pub tasks: u32,
    pub done: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: AreaId,
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum View {
    Inbox,
    Today,
    Upcoming,
    Anytime,
    Someday,
    Project,
    Logbook,
    Trash,
}

impl View {
    pub fn as_str(self) -> &'static str {
        match self {
            View::Inbox => "inbox",
            View::Today => "today",
            View::Upcoming => "upcoming",
            View::Anytime => "anytime",
            View::Someday => "someday",
            View::Project => "project",
            View::Logbook => "logbook",
            View::Trash => "trash",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Density {
    Airy,
    Cozy,
    Dense,
}

impl Density {
    pub fn row_h(self) -> u16 {
        match self {
            Density::Airy => 44,
            Density::Cozy => 36,
            Density::Dense => 28,
        }
    }

    pub fn pad_y(self) -> u16 {
        match self {
            Density::Airy => 13,
            Density::Cozy => 9,
            Density::Dense => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Work,
    Home,
    Errand,
}

impl Filter {
    pub fn matches(&self, tags: &[String]) -> bool {
        match self {
            Filter::All => true,
            Filter::Work => tags.iter().any(|t| t == "@work"),
            Filter::Home => tags.iter().any(|t| t == "@home"),
            Filter::Errand => tags.iter().any(|t| t == "@errand"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateField {
    When,
    Deadline,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTarget {
    Single(TaskId),
    Bulk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorKind {
    MainCenter,
    Inspector,
    BulkBar,
}

#[derive(Debug, Clone)]
pub struct DatePickerState {
    pub target: DateTarget,
    pub field: DateField,
    pub anchor: AnchorKind,
    pub view_year: i32,
    pub view_month: u32,
    pub initial_value: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupKey(pub String);

impl GroupKey {
    pub fn new(s: impl Into<String>) -> Self {
        GroupKey(s.into())
    }
}

#[derive(Debug, Clone)]
pub struct DueChip {
    pub label: String,
    pub state: DueState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DueState {
    Overdue,
    Today,
    Scheduled,
}
