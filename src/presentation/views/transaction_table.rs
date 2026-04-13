use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, scrollable, text, Column};
use iced::{color, Background, Border, Element, Fill, Theme};

use crate::domain::AccountView;
use crate::presentation::message::Msg;
use crate::presentation::theme::BOLD;
use crate::presentation::views::transaction_detail::view_txn_detail;

pub fn view_table<'a>(acct: &'a AccountView, expanded: Option<usize>) -> Element<'a, Msg> {
    let w_date: f32 = 86.0;
    let w_type: f32 = 66.0;
    let w_name = Fill;
    let w_memo = Fill;
    let w_amt: f32 = 100.0;

    let header = container(
        row![
            text("Date").size(10).font(BOLD).width(w_date),
            text("Type").size(10).font(BOLD).width(w_type),
            text("Name / Payee").size(10).font(BOLD).width(w_name),
            text("Memo").size(10).font(BOLD).width(w_memo),
            container(
                text("Amount")
                    .size(10)
                    .font(BOLD)
                    .align_x(Horizontal::Right),
            )
            .width(w_amt)
            .align_x(Horizontal::Right),
        ]
        .spacing(6)
        .padding([8, 14])
        .align_y(iced::Alignment::Center),
    )
    .width(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(color!(0xf1f3f5))),
        border: Border {
            color: color!(0xe5e7eb),
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    });

    let rows =
        acct.transactions
            .iter()
            .enumerate()
            .fold(Column::new(), |col, (i, txn)| {
                let bg = if i % 2 == 0 {
                    color!(0xffffff)
                } else {
                    color!(0xfafbfc)
                };
                let amt_color = if txn.raw_amount.is_sign_negative() {
                    color!(0xff3b30)
                } else {
                    color!(0x34c759)
                };

                let main_row = button(
                    row![
                        text(&txn.date).size(11).width(w_date),
                        text(&txn.txn_type)
                            .size(11)
                            .width(w_type)
                            .color(color!(0x666666)),
                        text(&txn.name).size(11).width(w_name),
                        text(&txn.memo)
                            .size(11)
                            .width(w_memo)
                            .color(color!(0x999999)),
                        container(
                            text(&txn.amount_str)
                                .size(11)
                                .color(amt_color)
                                .align_x(Horizontal::Right),
                        )
                        .width(w_amt)
                        .align_x(Horizontal::Right),
                    ]
                    .spacing(6)
                    .padding([6, 14])
                    .align_y(iced::Alignment::Center),
                )
                .width(Fill)
                .style(move |_: &Theme, _status| button::Style {
                    background: Some(Background::Color(bg)),
                    border: Border::default(),
                    shadow: iced::Shadow::default(),
                    text_color: color!(0x1d1d1f),
                })
                .on_press(Msg::ToggleRow(i));

                let col = col.push(main_row);

                if expanded == Some(i) {
                    col.push(view_txn_detail(txn))
                } else {
                    col
                }
            });

    container(
        column![header, scrollable(rows.width(Fill)).height(Fill),]
            .width(Fill)
            .height(Fill),
    )
    .width(Fill)
    .height(Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(color!(0xffffff))),
        ..Default::default()
    })
    .into()
}
