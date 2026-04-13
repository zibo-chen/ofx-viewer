use iced::font;
use iced::widget::container;
use iced::{color, Background, Border, Element, Font, Shadow, Vector};

use crate::presentation::message::Msg;

pub const BOLD: Font = Font {
    weight: font::Weight::Bold,
    ..Font::DEFAULT
};

pub const SIDEBAR_WIDTH: f32 = 280.0;

pub fn card<'a>(content: impl Into<Element<'a, Msg>>) -> Element<'a, Msg> {
    container(content)
        .padding(14)
        .width(iced::Fill)
        .style(|_: &iced::Theme| container::Style {
            background: Some(Background::Color(color!(0xffffff))),
            border: Border {
                color: color!(0xe5e7eb),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.03),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 4.0,
            },
            ..Default::default()
        })
        .into()
}
