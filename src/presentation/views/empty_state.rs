use iced::alignment::Horizontal;
use iced::widget::{button, center, column, container, image, text};
use iced::{color, Background, Element, Fill, Theme};

use crate::presentation::message::Msg;

const LOGO_BYTES: &[u8] = include_bytes!("../../../res/logo.png");

pub fn view_empty(error: Option<&str>) -> Element<'_, Msg> {
    let content = if let Some(err) = error {
        column![
            text("⚠️").size(48),
            text("Failed to parse OFX file")
                .size(18)
                .font(super::super::theme::BOLD),
            text(err).size(13).color(color!(0xff3b30)),
        ]
        .spacing(8)
        .align_x(Horizontal::Center)
    } else {
        let logo = image(image::Handle::from_bytes(LOGO_BYTES))
            .width(96)
            .height(96);
        column![
            logo,
            text("Open an OFX file to get started")
                .size(18)
                .color(color!(0x86868b)),
            text("Supports .ofx and .qfx bank / credit card statements")
                .size(13)
                .color(color!(0xaeaeb2)),
        ]
        .spacing(8)
        .align_x(Horizontal::Center)
    };

    center(
        column![
            content,
            button(text("📂  Open File").size(14),)
                .padding([10, 24])
                .style(|theme: &Theme, status| {
                    let mut s = button::primary(theme, status);
                    s.border.radius = 8.0.into();
                    s
                })
                .on_press(Msg::Open),
        ]
        .spacing(20)
        .align_x(Horizontal::Center),
    )
    .width(Fill)
    .height(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(color!(0xf8f9fb))),
        ..Default::default()
    })
    .into()
}
