use crate::gui::theme::Palette;
use iced::widget::container;
use iced::{Background, Theme};

pub fn panel<'a>(
    content: impl Into<iced::Element<'a, crate::gui::app::Message>>,
) -> container::Container<'a, crate::gui::app::Message> {
    container(content).style(|_theme: &Theme| {
        let palette = Palette::DARK;
        container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.text),
            ..Default::default()
        }
    })
}
