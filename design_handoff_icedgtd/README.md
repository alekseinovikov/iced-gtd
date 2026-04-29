# Handoff: IcedGTD — Get Things Done desktop app

## Overview

IcedGTD is a calm, minimal desktop GTD (Getting Things Done) task manager. Three-pane layout: **collapsible sidebar (Areas → Projects)** · **grouped task list with inline editing** · **detail inspector**. Light + dark themes, three density modes, frosty cyan accent.

The application is intended to be implemented in **Rust using the [Iced](https://iced.rs) framework**.

## About the Design Files

The files in this bundle are **design references created in HTML/JSX** — interactive prototypes showing the intended look, layout, and behavior. They are **not** production code to port directly.

Your task is to **recreate the designs in Rust + Iced** using Iced's widget tree, message-based architecture, and theming system. The HTML/CSS exists only to communicate visual + interaction intent. Treat it like a Figma file — extract spacing, colors, typography, behavior, then rebuild with idiomatic Iced widgets.

## Fidelity

**High-fidelity.** Colors, typography, spacing, radii, and shadows are final. Match them in Iced via:
- Custom `Theme` with the palette below.
- `iced::widget::Container` styles for cards / dividers.
- Custom widget styles (`button::Style`, `text_input::Style`, `scrollable::Style`).
- `iced::Font` for Inter Tight + JetBrains Mono (load via `Font::with_name` and bundle the TTFs).

## Application architecture (Iced)

Suggested message-based architecture (`iced 0.13+`):

```rust
pub struct App {
    tasks: Vec<Task>,
    areas: Vec<Area>,
    view: View,                  // Inbox | Today | Upcoming | Anytime | Someday | Project(Id) | Logbook | Trash
    active_project: Option<ProjectId>,
    open_task: Option<TaskId>,
    selection: HashSet<TaskId>,
    show_sidebar: bool,
    show_inspector: bool,
    date_picker: Option<DatePickerState>,
    theme_mode: ThemeMode,       // Light | Dark
    density: Density,            // Airy | Cozy | Dense
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectView(View),
    SelectProject(ProjectId),
    ToggleArea(AreaId),
    OpenTask(TaskId),
    CheckTask(TaskId),
    ToggleSelect(TaskId),
    AddTask { group: GroupKey, title: String },
    EditTaskTitle(TaskId, String),
    EditTaskNotes(TaskId, String),
    ScheduleTask(TaskId, Option<NaiveDate>),
    SetDeadline(TaskId, Option<NaiveDate>),
    ToggleChecklistItem(TaskId, ChecklistId),
    AddChecklistItem(TaskId, String),
    ReorderTask { from: TaskId, to: TaskId },
    BulkComplete,
    BulkSchedule(Option<NaiveDate>),
    BulkDelete,
    OpenDatePicker { task: TaskId, field: DateField, anchor: Rectangle },
    CloseDatePicker,
    ToggleSidebar,
    ToggleInspector,
    SetTheme(ThemeMode),
    SetDensity(Density),
}
```

Use `iced::widget::row!` for the three-pane layout, `column!` inside each pane, `scrollable` for the sidebar + task list + inspector body. Use `iced::overlay` for the date picker popover and the bulk-action floating bar.

---

## Screens / Views

The app is one window with one consistent shell; the **right two panes change** based on the selected view. Below describes the shell, then each view variant.

### Window shell (always present)

- **Window size**: default `1320 × 860` px, min `980 × 620`.
- **Window radius**: 14 px (use `iced::window::Settings { decorations: false, transparent: true }` and a custom rounded container if you want the rounded shell, otherwise use OS-native chrome).
- **Titlebar** (38 px tall, draggable):
  - Left: 3 mock window dots (13 × 13 px circles) + sidebar toggle icon.
  - Center: "❄ **IcedGTD** · built with Iced.rs" — uppercase tracking 0.04em, 12 px, ink-3.
  - Right: theme toggle (sun/moon icon) + inspector toggle.
- **Layout grid**: `248 px sidebar | 1fr main | 320 px inspector`. When inspector hidden: `248 | 1fr`. When sidebar hidden too: `0 | 1fr | 320` etc.

### 1. Sidebar (248 px, light bg-2)

Top-down:

1. **Search box** — 30 px tall, 7 px radius, surface bg, `magnifier` icon left, "⌘K" kbd pill right. Placeholder "Search".
2. **Top nav** — vertical list, each item 30 px tall, 7 px radius, columns `[16 px icon] [label] [count badge]`:
   - Inbox · Today · Upcoming · Anytime · Someday
   - Active item: `--accent-soft` bg, `--accent-ink` text (light) / `--accent` text (dark).
   - Hover: `--bg-3` bg.
3. **Section label "AREAS"** — 10.5 px uppercase tracked label, ink-4. Right-side `+` button to create a new area.
4. **Areas list** — for each area:
   - **Area header** (28 px tall): `[chevron] [8×8 accent square] [area name]` — bold 12.5 px. Click to expand/collapse children. Chevron rotates -90° when collapsed (transition 0.15s).
   - **Project list** (indented 22 px, with a 1 px left guide line): each project is `[12 px progress ring] [project name] [open-count]`. Progress ring: a `conic-gradient` from 0 → `done/total` × 100%. Use `iced::widget::Canvas` for the ring or render an SVG.
5. **Section label "ARCHIVE"** — same style, contains Logbook, Trash.
6. **Footer** (border-top): 28 px circular gradient avatar + name "Sara Kjartansdóttir" + sub "Synced · just now" + 3-dot menu button.

### 2. Main pane (center)

Always shows:

- **Header** (padding `22 32 12`):
  - 10×10 px accent square + page title (Inter Tight 26 px / 600 / -0.015em letter-spacing) + meta text (13 px ink-3).
  - Sub-line below (13 px ink-3) — varies per view.
  - Right: ghost more-button + primary "**+ New task** ⌘N" button.
- **Toolbar** (40 px tall, border-bottom):
  - Left: filter chips (`All` active, `@work`, `@home`, `@errand`). Chips: 24 px tall, 999 px radius, 1 px border. Active chip = solid ink bg, bg text.
  - Right: "N tasks" count + ⌘F kbd hint.
- **Task list** (scrollable, padding `8 24 80`):
  - **Section divider** (when group has a title): 11.5 px uppercase tracked label, optional subtitle in normal case, with a 1 px hairline filling remaining row width.
  - **Task rows** — see below.
  - **Inline composer** at end of group when `allowAdd` is true.
- **Bulk action bar** — appears bottom-center when selection is non-empty.

#### Task row

Grid `24 | 22 | 1fr | auto | auto` columns, `min-height` per density: airy 44 / cozy 36 / dense 28 px. Padding: `9 8` (cozy). 8 px radius. Border 1 px transparent (becomes accent on selected).

- **Drag grip** (24 col): 6 dot grid, opacity 0 by default, opacity 1 on hover. `cursor: grab`.
- **Checkbox** (22 col): 18 × 18 px, 4 px radius, 1.5 px border ink-4.
  - Hover: border ink-2.
  - Checked: filled accent, white check icon, scale-in animation (`cubic-bezier(.5,1.6,.4,1)` 0.2s).
  - Deadline (overdue): warn-colored border.
  - On check: 0.5 s "burst" — an absolutely-positioned `::after` ring scales 0.4 → 1.6 and fades out.
- **Title** (1fr): 14 px ink. Done state: line-through + ink-4.
  - Optional after the label: 4 px notes-dot (if `notes` set), checklist mini-pill `✓ 1/3` (11 px bg-2 chip).
- **Meta** (auto): 12 px ink-3, gap 10 px:
  - Repeat icon (if repeating)
  - First tag pill (`@work` etc.) — 11 px bg-3 chip, 1 px line-soft border, 999 radius.
  - Project tag — 11.5 px ink-3, 6×6 accent square dot before label.
  - Due chip — calendar icon + relative date ("Today" / "Tomorrow" / "Yesterday" / "3d overdue" / "Sat" / "Mar 14").
    - Today → accent-ink (light) / accent (dark)
    - Overdue → danger
    - Future → ink-3
- **Drop indicator** when dragged-over: 2 px accent line at row top.
- **Multi-select**: cmd/ctrl-click toggles selection (accent-soft bg, accent-tinted border). Plain click opens task in inspector.

#### Inline composer (`.new-task`)

Same grid as task row. Initially: dashed border, ink-4 placeholder text "Add a task…". On focus: solid accent border, surface bg, shadow-1, hint text "↵ save · esc cancel" on the right. Enter creates task; Esc clears.

#### Bulk action bar

Position: `absolute`, `bottom: 22 px`, horizontally centered.

- Pill: ink bg / bg text, 999 radius, padding `6 8 6 16`, shadow-3.
- Layout: `[count pill][selected text][sep][complete][schedule][move][trash][sep][close]`.
- Count pill: solid accent, dark text, 12 px, 999 radius.
- Buttons: 30 × 30 px circular, transparent, hover bg `bg/15%`.

### 3. Inspector (320 px, right)

Visible only when a task is open AND `show_inspector` is true.

- **Header** (padding `18 20 12`, border-bottom):
  - Crumbs: `● Area / Project` in 11.5 px uppercase tracked, ink-4. Close button on the right.
  - Title: contentEditable `<h2>` — Inter Tight 18 px / 600 / -0.01em / line-height 1.3.
- **Body** (scrollable, padding `14 20 24`):
  - **Field rows** — grid `88 px label | 1fr value`, padding `7 0`, 13 px:
    - Label: 11.5 px uppercase tracked, ink-4.
    - Value: 13 px ink-2. Empty values render in ink-4.
    - Fields: When (date), Deadline (flag icon), Repeat, Project (project tag pill), Tags (tag pills + `+ Tag` chip).
  - **Notes** section: section title (11 px uppercase tracked ink-4) + textarea (surface bg, 1 px line border, 8 px radius, padding 12 px, 13 px / 1.55).
  - **Checklist** section: list of items. Each item is `[14×14 checkbox] [label]`. Done items: line-through ink-4. Bottom row is "Add subtask…" input.
  - **Activity** section: 12 px ink-4, line-height 1.7, e.g. "Created · 3 days ago".

### 4. Date picker popover

Floating popover anchored to the clicked element; render via `iced::overlay`.

- 12 px radius, surface bg, 1 px line border, shadow-3, padding 12 px, min 280 px wide.
- **Quick-row grid** (2 × 2): Today / Tomorrow / This weekend / Someday. Each button is 30 px tall, `[icon] [label] [day-hint]`. Hover bg-3.
- **Month nav**: `‹` button, month name (e.g. "March 2026"), `›` button.
- **Day grid**: 7 columns. Cells 30 px square, 6 px radius, hover bg-3.
  - Other-month cells: ink-4.
  - Today: accent border + accent-ink text.
  - Selected: solid accent bg + dark text.
- Footer: "Clear" + "Close" ghost buttons.

---

## Interactions & Behavior

| Interaction | Behavior |
|---|---|
| Click task row | Opens task in inspector |
| Cmd/Ctrl-click row | Toggles selection (multi-select) |
| Click checkbox | Toggles done. Plays burst animation (0.5 s ring) and check scale-in (0.2 s). |
| Drag row | Reorders within current view. 2 px accent indicator above drop target. |
| Click due-date pill in row | Opens date picker anchored to pill |
| Click "+ New task" | Spawns new task and opens it in inspector |
| Click inline composer | Focuses input. Enter = save & re-focus. Esc = clear & blur. |
| Toggle area chevron | Collapses/expands its project list (-90° rotation, 0.15s) |
| Theme toggle in titlebar | Swap `data-theme` attribute → flips palette via CSS vars. In Iced: rebuild Theme. |
| Selection > 0 | Bulk action bar slides in at bottom-center. Buttons: complete all, schedule all, move, delete, close. |
| `⌘K` | Focus search (not yet implemented in prototype) |
| `⌘N` | New task |
| `⌘F` | Filter (placeholder) |

### Animations

- **Checkbox burst**: 0.5 s ease-out. Scale 0.4 → 1.6, opacity 0.7 → 0. Use `iced::animation` or a `Subscription::frames` driver.
- **Check scale-in**: cubic-bezier(.5, 1.6, .4, 1) over 0.2 s.
- **Chevron rotation**: 0.15 s linear.
- **Hover transitions**: 0.15 s on color/background.

---

## State Management

In Iced you'd use `update(&mut self, message: Message) -> Task<Message>`. Persistence: serialize `tasks` to JSON via `serde` and write to the user's config dir (`dirs::config_dir()` + `icedgtd/store.json`). Watch with `tokio::time::interval` if needed.

Date computations: use `chrono::NaiveDate`. "Today" / "Tomorrow" / "3d overdue" formatting helpers should live in a `format_date(d: NaiveDate, today: NaiveDate) -> String` util.

---

## Design Tokens

### Colors (light theme)

| Token | Value | Use |
|---|---|---|
| `--bg`        | `oklch(99% 0.003 220)`   | Main canvas |
| `--bg-2`      | `oklch(97% 0.005 220)`   | Sidebar, inspector |
| `--bg-3`      | `oklch(94.5% 0.006 220)` | Hover surface |
| `--surface`   | `oklch(100% 0 0)`        | Cards, inputs, popovers |
| `--line`      | `oklch(91% 0.006 220)`   | Borders |
| `--line-soft` | `oklch(94% 0.005 220)`   | Subtle dividers |
| `--ink`       | `oklch(20% 0.01 240)`    | Primary text |
| `--ink-2`     | `oklch(40% 0.012 240)`   | Secondary text |
| `--ink-3`     | `oklch(58% 0.012 240)`   | Tertiary / meta |
| `--ink-4`     | `oklch(72% 0.01 240)`    | Placeholder, disabled |
| `--accent`    | `oklch(72% 0.12 200)`    | **Frosty cyan accent** |
| `--accent-2`  | `oklch(82% 0.09 200)`    | Hover variant |
| `--accent-soft`| `oklch(95% 0.04 200)`   | Selection / active row bg |
| `--accent-ink`| `oklch(35% 0.08 220)`    | Text on accent-soft |
| `--danger`    | `oklch(62% 0.18 25)`     | Overdue |
| `--warn`      | `oklch(72% 0.14 70)`     | Deadline checkbox border |

### Colors (dark theme)

| Token | Value |
|---|---|
| `--bg`        | `oklch(15% 0.008 240)` |
| `--bg-2`      | `oklch(18% 0.009 240)` |
| `--bg-3`      | `oklch(21% 0.01 240)`  |
| `--surface`   | `oklch(20% 0.009 240)` |
| `--line`      | `oklch(28% 0.01 240)`  |
| `--line-soft` | `oklch(24% 0.009 240)` |
| `--ink`       | `oklch(95% 0.005 220)` |
| `--ink-2`     | `oklch(82% 0.008 220)` |
| `--ink-3`     | `oklch(64% 0.01 220)`  |
| `--ink-4`     | `oklch(48% 0.01 220)`  |
| `--accent`    | `oklch(78% 0.13 200)`  |
| `--accent-soft`| `oklch(28% 0.06 210)` |

In Iced, convert OKLCH → linear sRGB using `palette::Oklch::to_srgb()` (the `palette` crate).

### Spacing scale

`4 · 8 · 10 · 12 · 14 · 16 · 18 · 20 · 22 · 24 · 32` px. Window padding 24 px. Pane horizontal padding 32 px main / 20 px inspector / 12 px sidebar.

### Density scale (row height / vertical padding)

| Mode | Row height | Padding y |
|---|---|---|
| Airy  | 44 px | 13 px |
| Cozy  | 36 px | 9 px |
| Dense | 28 px | 5 px |

### Typography

| Family | Use | Size & weight |
|---|---|---|
| Inter Tight | UI | 13–14 px / 400–500 body, 18–26 px / 600 titles, letter-spacing -0.01em on display |
| JetBrains Mono | Kbd hints, counts, debug | 11–12 px / 500 |

Section labels: 10.5–11.5 px uppercase, letter-spacing 0.06–0.08em, ink-4.

### Radius

| Element | Radius |
|---|---|
| Window | 14 px |
| Buttons / inputs / chips (rect) | 7 px |
| Task rows / cards | 8 px |
| Popovers | 12 px |
| Pills, count badges | 999 px |
| Checkbox | 4 px |
| Tiny progress ring | 50% |

### Shadows

```
shadow-1: 0 1px 0 rgba(0,0,0,0.04), 0 1px 2px rgba(0,0,0,0.04)
shadow-2: 0 6px 18px rgba(0,0,0,0.07), 0 2px 6px rgba(0,0,0,0.05)
shadow-3: 0 18px 50px rgba(0,0,0,0.13), 0 4px 12px rgba(0,0,0,0.07)
```

In Iced, `Shadow { color, offset, blur_radius }`. For multi-layer shadows, stack two `Container` layers.

---

## Iced-specific implementation tips

- **Three-pane layout**: `row![sidebar, vertical_rule, main, vertical_rule, inspector]`.
- **Sidebar collapse animation**: animate `width` via `Animation` (in iced 0.13+) or interpolate manually with `Subscription::frames`.
- **Drag-to-reorder**: there's no built-in drag widget. Implement with custom widget that captures `Event::Mouse` events and emits a `Message::ReorderTask` on drop. Use `mouse::Cursor` position vs row layout bounds.
- **Date picker popover**: use `iced::overlay::Element` so it floats above other widgets and dismisses on outside click.
- **Editable title**: `text_input` with no border styling for inspector heading.
- **Checkbox burst**: render via `Canvas` widget with an animated radius/alpha, or two stacked `Container`s.
- **Theme switching**: keep `Theme::Light` and `Theme::Dark` as full custom palettes; expose via your `App::theme()` method.
- **Custom fonts**: `application().font(include_bytes!("../assets/InterTight.ttf"))`.

---

## Assets

- **Fonts** — Inter Tight + JetBrains Mono. Both are open licensed; download TTFs from Google Fonts and bundle.
- **Icons** — All icons in the prototype are inline Feather-style SVG strokes (1.5–2 px stroke). For Iced, either:
  - Use the `iced::widget::svg` widget with extracted SVG files in `assets/icons/`, or
  - Use a font-icon set like Lucide/Phosphor and the `iced` text widget.
- **No raster images required.**

---

## Files in this bundle

| File | What it is |
|---|---|
| `IcedGTD.html` | Entry HTML — boots the prototype |
| `styles.css` | All visual tokens, layout, animations — **the source of truth for styling** |
| `app.jsx` | App state machine + view router (`buildGroups`, `getHeaderInfo`) |
| `sidebar.jsx` | Sidebar with collapsible Areas → Projects |
| `tasklist.jsx` | Task list, task row, inline composer, drag-reorder |
| `inspector.jsx` | Detail panel — fields, notes, checklist, activity |
| `datepicker.jsx` | Calendar popover with quick rows |
| `data.jsx` | Sample data + icon set |
| `tweaks-panel.jsx` | Floating tweaks panel (theme + density) — drop in production |

Open `IcedGTD.html` in a browser to interact with the live prototype. Inspect elements / view source to confirm any measurement.

---

## Out of scope / next-pass ideas

- Quick-capture global hotkey overlay (⌘⇧Space)
- Full RRULE-based repeating-task editor
- Search results screen
- Calendar/agenda month view
- Sync / multi-device

These can ship after v1.
