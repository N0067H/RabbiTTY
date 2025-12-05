use iced::widget::{Column, button, column, text};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
pub struct Counter {
    val: i32,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.val).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.val += 1;
            }
            Message::Decrement => {
                self.val -= 1;
            }
        }
    }
}
