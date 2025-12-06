use crate::gui::theme::{Palette, RADIUS_NORMAL};
use iced::widget::text_input;
use iced::{Background, Border, Color, Theme};

pub fn default<'a>(
    placeholder: &str,
    value: &str,
) -> text_input::TextInput<'a, crate::gui::app::Message> {
    text_input(placeholder, value)
        .style(|_theme: &Theme, status: text_input::Status| {
            let palette = Palette::DARK;
            let base_border = Border {
                radius: RADIUS_NORMAL.into(),
                width: 1.0,
                color: Color::TRANSPARENT,
            };

            let active = text_input::Style {
                background: Background::Color(palette.surface),
                border: Border {
                    color: Color {
                        a: 0.1,
                        ..palette.text
                    },
                    ..base_border
                },
                icon: palette.text_secondary,
                placeholder: palette.text_secondary,
                value: palette.text,
                selection: Color {
                    a: 0.2,
                    ..palette.accent
                },
            };

            match status {
                text_input::Status::Active => active,
                text_input::Status::Hovered => text_input::Style {
                    border: Border {
                        color: Color {
                            a: 0.3,
                            ..palette.text
                        },
                        ..active.border
                    },
                    ..active
                },
                text_input::Status::Focused => text_input::Style {
                    border: Border {
                        color: palette.accent,
                        ..active.border
                    },
                    ..active
                },
                text_input::Status::Disabled => text_input::Style {
                    background: Background::Color(Color {
                        a: 0.5,
                        ..palette.surface
                    }),
                    value: palette.text_secondary,
                    ..active
                },
            }
        })
        .padding(10)
}
