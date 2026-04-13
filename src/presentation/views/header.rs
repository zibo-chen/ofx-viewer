use iced::widget::{button, container, horizontal_space, image, row, text};
use iced::{color, Background, Border, Element, Fill, Shadow, Theme, Vector};

use crate::presentation::message::Msg;
use crate::presentation::theme::BOLD;

const LOGO_BYTES: &[u8] = include_bytes!("../../../res/logo.png");

pub fn view_header(file_name: Option<&str>) -> Element<'_, Msg> {
    let logo = image(image::Handle::from_bytes(LOGO_BYTES))
        .width(28)
        .height(28);
    let title = text("OFX Viewer").size(18).font(BOLD);

    let open_btn = button(
        row![text("📂").size(14), text(" Open").size(13)]
            .spacing(4)
            .align_y(iced::Alignment::Center),
    )
    .padding([6, 16])
    .style(|theme: &Theme, status| {
        let mut s = button::primary(theme, status);
        s.border.radius = 6.0.into();
        s
    })
    .on_press(Msg::Open);

    let file_label = text(file_name.unwrap_or("No file loaded"))
        .size(12)
        .color(color!(0x86868b));

    container(
        row![logo, title, horizontal_space(), open_btn, file_label]
            .spacing(12)
            .align_y(iced::Alignment::Center)
            .padding([10, 20]),
    )
    .width(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(color!(0xffffff))),
        border: Border {
            color: color!(0xe5e7eb),
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 4.0,
        },
        ..Default::default()
    })
    .into()
}
