use iced::widget::{button, column, container, horizontal_rule, row, scrollable, text, Column};
use iced::{color, Background, Border, Element, Fill, Length, Theme};

use crate::domain::AccountView;
use crate::infrastructure::fmt_dec;
use crate::presentation::message::Msg;
use crate::presentation::theme::{card, BOLD, SIDEBAR_WIDTH};

pub fn view_sidebar<'a>(accounts: &'a [AccountView], tab: usize) -> Element<'a, Msg> {
    let idx = tab.min(accounts.len().saturating_sub(1));
    let acct = &accounts[idx];

    let mut sidebar = Column::new().spacing(10).padding([14, 14]);

    // Account tabs (if multiple)
    if accounts.len() > 1 {
        let mut tabs = Column::new().spacing(4);
        for (i, a) in accounts.iter().enumerate() {
            let active = i == idx;
            let label = format!("{} ({})", a.label, a.account_id);
            let btn = button(text(label).size(11))
                .padding([5, 10])
                .width(Fill)
                .style(move |theme: &Theme, status| {
                    let mut s = if active {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    };
                    s.border.radius = 5.0.into();
                    s
                })
                .on_press(Msg::Tab(i));
            tabs = tabs.push(btn);
        }
        sidebar = sidebar.push(tabs);
        sidebar = sidebar.push(horizontal_rule(1));
    }

    // Account info
    sidebar = sidebar.push(view_account_info(acct));

    // Balances
    if acct.ledger.is_some() || acct.available.is_some() {
        sidebar = sidebar.push(view_balances(acct));
    }

    // Transaction stats
    sidebar = sidebar.push(view_stats(acct));

    // Extra balance list
    if !acct.balance_list.is_empty() {
        sidebar = sidebar.push(view_balance_list(acct));
    }

    container(scrollable(sidebar).height(Fill))
        .width(Length::Fixed(SIDEBAR_WIDTH))
        .height(Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(color!(0xf8f9fb))),
            border: Border {
                color: color!(0xe5e7eb),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}

fn view_account_info(acct: &AccountView) -> Element<'_, Msg> {
    let mut col = Column::new().spacing(3);
    col = col.push(text(&acct.label).size(10).color(color!(0x86868b)));
    col = col.push(text(&acct.account_id).size(22).font(BOLD));
    col = col.push(
        text(format!("Full ID: {}", acct.account_id_full))
            .size(9)
            .color(color!(0xaeaeb2)),
    );
    if let Some(ref bid) = acct.bank_id {
        col = col.push(
            text(format!("Bank: {bid}"))
                .size(9)
                .color(color!(0xaeaeb2)),
        );
    }
    if let Some(ref br) = acct.branch_id {
        col = col.push(
            text(format!("Branch: {br}"))
                .size(9)
                .color(color!(0xaeaeb2)),
        );
    }
    if let Some(ref org) = acct.fi_org {
        col = col.push(text(org.as_str()).size(10).color(color!(0x86868b)));
    }
    if let Some(ref fid) = acct.fi_id {
        col = col.push(
            text(format!("FI ID: {fid}"))
                .size(9)
                .color(color!(0xaeaeb2)),
        );
    }
    if let Some(ref ak) = acct.account_key {
        col = col.push(
            text(format!("Key: {ak}"))
                .size(9)
                .color(color!(0xaeaeb2)),
        );
    }
    col = col.push(
        text(format!(
            "{} · OFX {} · {}",
            acct.currency, acct.ofx_version, acct.language
        ))
        .size(9)
        .color(color!(0xaeaeb2)),
    );
    col = col.push(
        text(format!("Server: {}", acct.server_date))
            .size(9)
            .color(color!(0xaeaeb2)),
    );
    if let Some(ref uid) = acct.txn_uid {
        col = col.push(
            text(format!("TXN UID: {uid}"))
                .size(9)
                .color(color!(0xaeaeb2)),
        );
    }
    card(col)
}

fn view_balances(acct: &AccountView) -> Element<'_, Msg> {
    let mut col = Column::new().spacing(8);

    if let Some(ref b) = acct.ledger {
        let c = if b.raw.is_sign_negative() {
            color!(0xff3b30)
        } else {
            color!(0x1d1d1f)
        };
        col = col.push(
            column![
                text("Ledger Balance").size(10).color(color!(0x86868b)),
                text(&b.display).size(20).font(BOLD).color(c),
                text(format!("as of {}", b.as_of))
                    .size(9)
                    .color(color!(0xaeaeb2)),
            ]
            .spacing(2),
        );
    }

    if acct.ledger.is_some() && acct.available.is_some() {
        col = col.push(horizontal_rule(1));
    }

    if let Some(ref b) = acct.available {
        let c = if b.raw.is_sign_negative() {
            color!(0xff3b30)
        } else {
            color!(0x1d1d1f)
        };
        col = col.push(
            column![
                text("Available Balance").size(10).color(color!(0x86868b)),
                text(&b.display).size(20).font(BOLD).color(c),
                text(format!("as of {}", b.as_of))
                    .size(9)
                    .color(color!(0xaeaeb2)),
            ]
            .spacing(2),
        );
    }

    card(col)
}

fn view_stats(acct: &AccountView) -> Element<'_, Msg> {
    let n = acct.transactions.len();
    let range = acct
        .date_range
        .as_ref()
        .map_or_else(|| "—".into(), |(s, e)| format!("{s}  →  {e}"));

    card(
        column![
            text("Transactions").size(10).color(color!(0x86868b)),
            text(n.to_string()).size(22).font(BOLD),
            text(range).size(9).color(color!(0xaeaeb2)),
            row![
                text(format!("↑ {}", fmt_dec(acct.total_credit)))
                    .size(11)
                    .color(color!(0x34c759)),
            ],
            row![
                text(format!("↓ {}", fmt_dec(acct.total_debit.abs())))
                    .size(11)
                    .color(color!(0xff3b30)),
            ],
        ]
        .spacing(3),
    )
}

fn view_balance_list(acct: &AccountView) -> Element<'_, Msg> {
    let mut col = Column::new().spacing(4);
    col = col.push(text("Balance List").size(10).color(color!(0x86868b)));
    for b in &acct.balance_list {
        let mut line = format!("{}: {} ({})", b.name, b.value, b.kind);
        if let Some(ref dt) = b.as_of {
            line.push_str(&format!(" @ {dt}"));
        }
        if let Some(ref cur) = b.currency {
            line.push_str(&format!(" [{cur}]"));
        }
        col = col.push(text(line).size(10));
        if !b.description.is_empty() {
            col = col.push(text(&b.description).size(9).color(color!(0xaeaeb2)));
        }
    }
    card(col)
}
