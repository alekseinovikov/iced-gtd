mod app;
mod data;
mod date;
mod models;
mod storage;
mod styles;
mod theme;
mod ui;

use iced::window;
use iced::Size;

use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("IcedGTD")
        .theme(App::theme)
        .subscription(App::subscription)
        .window(window::Settings {
            size: Size::new(1320.0, 860.0),
            min_size: Some(Size::new(980.0, 620.0)),
            position: window::Position::Centered,
            decorations: false,
            transparent: true,
            ..Default::default()
        })
        .antialiasing(true)
        .run()
}
