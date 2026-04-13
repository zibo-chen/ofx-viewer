use iced::widget::{column, container, horizontal_rule, row, text, Column};
use iced::{color, Background, Border, Element, Fill, Theme};

use crate::domain::TxnRow;
use crate::presentation::message::Msg;
use crate::presentation::theme::BOLD;

pub fn view_txn_detail<'a>(txn: &'a TxnRow) -> Element<'a, Msg> {
    fn label(l: &str) -> iced::widget::Text<'_> {
        text(l).size(11).font(BOLD).color(color!(0x86868b))
    }
    fn value(v: &str) -> iced::widget::Text<'_> {
        text(v).size(11)
    }

    let mut left = Column::new().spacing(4);
    left = left.push(row![label("FIT ID"), value(&txn.fit_id)].spacing(8));
    if let Some(ref v) = txn.server_txn_id {
        left = left.push(row![label("Server TXN ID"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.check_number {
        left = left.push(row![label("Check #"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.reference_number {
        left = left.push(row![label("Ref #"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.payee_id {
        left = left.push(row![label("Payee ID"), value(v)].spacing(8));
    }

    let mut right = Column::new().spacing(4);
    if let Some(ref v) = txn.sic {
        right = right.push(row![label("SIC"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.correction_id {
        right = right.push(row![label("Correction ID"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.correction_action {
        right = right.push(row![label("Correction"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.currency_info {
        right = right.push(row![label("Currency"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.date_user {
        right = right.push(row![label("User Date"), value(v)].spacing(8));
    }
    if let Some(ref v) = txn.date_available {
        right = right.push(row![label("Available Date"), value(v)].spacing(8));
    }

    container(
        column![
            horizontal_rule(1),
            row![left, right].spacing(40).padding([8, 16]),
        ]
        .spacing(0),
    )
    .width(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(color!(0xf0f4ff))),
        border: Border {
            color: color!(0xd0d8f0),
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}
