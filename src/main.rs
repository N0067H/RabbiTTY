mod gui;

use crate::gui::App;

fn main() -> iced::Result {
    iced::application("foo", App::update, App::view)
        .subscription(App::subscription)
        .window(iced::window::Settings {
            exit_on_close_request: true,
            ..Default::default()
        })
        .run()
}
