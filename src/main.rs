mod gui;

use crate::gui::Counter;

#[tokio::main]
async fn main() {
    let _ = iced::run("foo", Counter::update, Counter::view);
}
