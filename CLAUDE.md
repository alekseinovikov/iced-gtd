# IcedGTD — Claude session knowledge

A Rust + **Iced 0.14** implementation of the design handoff in `design_handoff_icedgtd/`. The handoff (HTML/CSS/JSX prototype) is the source of truth for visuals; do not edit it. Open `design_handoff_icedgtd/IcedGTD.html` in a browser whenever you need to confirm visual intent.

## How to run / verify

- `cargo build` — should be 0 errors, ~20 warnings (most are dead-code stubs reserved for future phases).
- `cargo run` — opens a 1320×860 frameless rounded window with a 38 px custom titlebar, sidebar, main pane, inspector.
- Persistence side-effect: `~/Library/Application Support/icedgtd/store.json` is created/updated on every mutation. Delete it to reseed from `data.rs`.
- There is no test suite; verification is visual against `IcedGTD.html`.

## Architecture

Single-window app, Elm-style:
- `src/main.rs` boots `iced::application(App::new, App::update, App::view)` with `decorations: false`, `transparent: true`, custom rounded shell.
- `src/app.rs` holds **all** state (`App`), the `Message` enum, `update`, top-level `view` (composes the rounded shell + Stack overlays for bulk bar + date picker), `subscription`, and pure helpers `build_groups()` and `header_info()`.
- `src/models.rs` — `Task`, `Area`, `Project`, `View`, `Density`, `ThemeMode`, `Filter`, `DatePickerState`, etc. Most types derive `Serialize/Deserialize` for persistence.
- `src/data.rs` — seed data mirroring `design_handoff_icedgtd/data.jsx`. 17 tasks, 3 areas, 6 projects.
- `src/date.rs` — `format_date(d, today)` ports `tasklist.jsx::fmtDate` (Today / Tomorrow / `{n}d overdue` / weekday / `MMM d`). `due_state()`, `next_saturday()`.
- `src/storage.rs` — `Store` schema + async `load()`/`save()` via tokio fs. Persists tasks, areas, theme_mode, density, collapsed_areas. Save is gated on `App.initial_data_loaded` to avoid clobbering with seed before load completes.
- `src/theme/{mod.rs,oklch.rs}` — `Tokens` struct (the full palette: bg/bg_2/bg_3/surface/line/line_soft/ink/ink_2..4/accent/accent_2/accent_soft/accent_ink/accent_border/danger/warn/on_accent/shadows). OKLCH→sRGB conversion via the `palette` crate. `Tokens::light()` / `Tokens::dark()`.
- `src/styles/{button.rs, container.rs, text_input.rs, scrollable.rs}` — every custom widget style is a function `pub fn name(tokens: Tokens) -> impl Fn(&Theme, Status) -> Style`. Tokens is `Copy`, so closures capture by value.
- `src/ui/` — view modules: `titlebar`, `sidebar`, `main_pane` (header + toolbar + scroll list + bulk bar overlay), `task_list`, `task_row`, `inspector`, `date_picker`, `checkbox` (Canvas burst overlay), `progress_ring` (Canvas), `icons` (26 inlined SVGs via `include_bytes!` + `Cow::Borrowed`).
- `assets/icons/*.svg` — Feather-style icons extracted from `data.jsx` and standardized to `viewBox="0 0 24 24"` with `currentColor` strokes. Accessed via `ui::icons::{name}()` returning `svg::Handle`.

## Iced 0.14 API gotchas (learned the hard way)

These don't match older docs / context7's older snippets. They cost real time on the first build pass — please use these directly:

- **`Space`**: `Space::new()` takes 0 args. Use `Space::new().width(L).height(L)`. `Space::with_width(L)` / `with_height(L)` **don't exist**.
- **`iced::application(...)`** first arg is a `BootFn` (a function returning `(State, Task<Message>)` or just `State`). It is **not** a title string. Set the title separately via `.title("IcedGTD")` (a static string works; a `Fn(&State) -> String` closure runs into HRTB inference issues — stick to the static string unless you really need dynamic).
- **Border radius**: `Border::radius` takes `Into<Radius>`. `From<f32> for Radius` exists (uniform), but `From<[f32; 4]>` does **not**. For per-corner radii, construct `iced::border::Radius { top_left, top_right, bottom_right, bottom_left }` directly.
- **Scrollable style**: `scrollable::Style` has fields `container`, `vertical_rail`, `horizontal_rail`, `gap: Option<Background>`, **and `auto_scroll: AutoScroll` (not Option)**. `AutoScroll` requires `background, border, shadow, icon` — provide all four. `scrollable::Scroller` has `background: Background` (not `color: Color`).
- **Widget IDs**: there is no `text_input::Id`. Use `iced::widget::Id`. `Id::new(&'static str)` is for static names; for runtime-built strings use `Id::from(String)` (or `string.into()`).
- **Focus**: `text_input::focus(...)` does not exist. Use `iced::widget::operation::focus(id)` — it returns a `Task<Message>` you return from `update`.
- **`event::listen_with`** takes a **fn pointer**, not a closure. You cannot capture state in it. If you need conditional behavior based on App state, do the conditional inside `update` and just always emit the message.
- **Window settings**: use `.window(window::Settings { size, min_size, position: Centered, decorations: false, transparent: true, .. })` on the application builder. `iced::application(...).window(...)` (not `.settings(Settings { window })`).
- **Window controls**: `iced::window::drag(id)`, `window::close(id)`, `window::minimize(id, true)` — all return `Task<Message>`. We get `id` via `window::open_events().map(Message::WindowOpened)` once at startup.
- **`text()` lifetimes**: `text(&local_string)` borrows the local for the returned `Element`'s lifetime, which then can't outlive the function. Pass owned `String` (clone it) to side-step lifetime errors. Same shape: `task_list::section_divider` and `task_row::view` deliberately take owned `String`s instead of borrowing from a loop-local `GroupedTasks`.
- **Hover styling on buttons**: `button::Status::Hovered` is a separate match arm from `Active`. We use it for cosmetic hover (e.g., `nav_item`).
- **Stack widget**: `iced::widget::stack![a, b]` layers from back to front. Used at App-root for bulk bar + date picker; used inside `task_row` to overlay the burst Canvas on the checkbox.
- **OKLCH → sRGB**: `oklch(L%, C, H_deg)` in `theme/oklch.rs` uses `palette::Oklch::new(l/100, c, h).into_color::<Srgb>()` then clamps to `Color::from_rgb`. This converts once at `Tokens::light()`/`dark()` construction, not per-render.

## Design tokens — hard rules

- **Match `design_handoff_icedgtd/styles.css` exactly** for spacing/radii/typography. Re-read it whenever a measurement is uncertain — it is the source of truth, not the README.
- Tokens flow tokens → style fns → widget. Don't hard-code colors in `ui/`. Add a token (or compute from one) if you need a new variant.
- Density (Airy/Cozy/Dense) drives row height + vertical padding via `Density::row_h()` / `pad_y()`. Use those getters; don't hard-code 36 px.

## Phase status

All 8 phases of `~/.claude/plans/read-design-handoff-icedgtd-readme-md-en-recursive-dove.md` shipped. Functional, with the gaps below.

## Known gaps (acknowledge before claiming "done")

- **Custom fonts not bundled**: the app uses Iced's default font. Inter Tight + JetBrains Mono TTFs need to be downloaded from Google Fonts, dropped into `assets/fonts/`, and added with `.font(include_bytes!(...))` on the application builder. The visual fidelity will jump noticeably once this is done.
- **No line-through for done tasks**: Iced text doesn't render `text-decoration: line-through`. Done tasks show in `ink-4` but without the strike. A future Canvas overlay can paint the line.
- **Chevron rotation and sidebar-collapse animations are instant**, not smoothly tweened. The `Tick` subscription is wired (gated on `bursting non-empty`) and `chevron_anims: HashMap<AreaId, f32>` exists in App state — only the interpolation + Canvas-rendered chevron is missing.
- **Checkbox scale-in is not animated** (the check icon appears immediately on toggle); the burst ring **is** animated.
- **Drag-to-reorder is hover-based, not position-based**: press grip → DragBegin, hover any row → DragHover sets `drop_target`, global mouse-up → DragEnd reorders. The dragged row does not get the 0.4-opacity "lifted" treatment. Drop indicator is a 2 px accent line above the target row. Works for slow movements; can mis-target if the cursor moves fast enough to outrun `mouse_area::on_enter` events.
- **Date picker is pane-relative**, not bounds-anchored to the clicked widget. From a row's due chip → centered above main pane; from inspector → aligned right of main pane; from bulk bar → centered above bulk bar. True bounds-anchoring needs a custom overlay widget (`iced::overlay::Element`).
- **Some `Message` variants are unused** (e.g., `OpenTask`, `AddTaskInGroup`, `ScheduleTaskField`, `OpenMenu`/`CloseMenu`). Kept as scaffolding for upcoming polish; the dead-code warnings are intentional. Don't remove without checking if they're queued for use.

## Where to find what

| Need | File |
|---|---|
| Add a new visual style | `src/styles/{button,container,text_input,scrollable}.rs` |
| Add a new widget tree | `src/ui/{module}.rs` and re-export in `src/ui/mod.rs` |
| Add new state | `App` struct in `src/app.rs` |
| Add a new message | `Message` enum in `src/app.rs` + match arm in `update` |
| Persist a new field | Add to `Store` in `src/storage.rs` + extend `App::snapshot()` and the `Loaded` arm |
| Change colors | `src/theme/mod.rs` `Tokens::light()` / `dark()` |
| Change density | `src/models.rs::Density::{row_h,pad_y}` |
| Date / due-state logic | `src/date.rs` |
| Group tasks for a view | `build_groups()` in `src/app.rs` (port of `app.jsx::buildGroups`) |
| Sample data | `src/data.rs` (matches `data.jsx`) |
| Add an icon | drop `name.svg` (viewBox 0 0 24 24, `currentColor`) into `assets/icons/` and add `icon_handle!(name, "name.svg");` in `src/ui/icons.rs` |

## Don't do these

- Don't bring in `iced_aw` or other addon crates without reason; vanilla Iced 0.14 covers everything we use.
- Don't introduce a `Theme::custom(name, Palette)` and try to hold all design tokens in it — Iced's `Palette` is too narrow (background/text/primary/success/danger). Tokens live in `App.tokens` and are read inside style closures. The `Theme` we return from `App::theme()` only signals Light vs Dark to Iced for its built-in defaults.
- Don't write style fns inline in view code. Put them in `src/styles/`.
- Don't compute layouts using widget bounds — Iced doesn't expose them. If you need anchor-aware positioning, write a custom overlay widget.
- Don't `Task::none()` after a mutation — return `self.persist()` instead. The save is async-spawned and gated on `initial_data_loaded`.
