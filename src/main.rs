mod app;
mod application;
mod domain;
mod infrastructure;
mod presentation;

use app::App;

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .window_size((1300.0, 850.0))
        .run_with(App::new)
}
