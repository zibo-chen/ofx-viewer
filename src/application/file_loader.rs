use rust_decimal::Decimal;

use crate::domain::{AccountView, BalanceInfo, BalanceItem, TxnRow};
use crate::infrastructure::{fmt_amt, fmt_dt, fmt_txn_type, mask};

pub async fn pick_file() -> Option<(String, String)> {
    let handle = rfd::AsyncFileDialog::new()
        .add_filter("OFX Files", &["ofx", "qfx"])
        .set_title("Open OFX File")
        .pick_file()
        .await?;
    let name = handle.file_name();
    let path = handle.path().to_path_buf();
    let bytes = std::fs::read(path).ok()?;
    let content = String::from_utf8_lossy(&bytes).into_owned();
    Some((name, content))
}

pub fn build_accounts(doc: &ofx_rs::OfxDocument) -> Vec<AccountView> {
    let signon = doc.signon();
    let fi_org = signon.fi_org().map(str::to_string);
    let fi_id = signon.fi_id().map(str::to_string);
    let server_date = fmt_dt(signon.date_time_server());
    let language = signon.language().to_string();
    let ver = doc.header().version();
    let ofx_version = format!("{}.{}.{}", ver.major(), ver.minor(), ver.patch());
    let mut out = Vec::new();

    if let Some(banking) = doc.banking() {
        for w in banking.statement_responses() {
            if let Some(s) = w.response() {
                let a = s.bank_account();
                let tl = s.transaction_list();
                let txns = extract_txns(tl);
                let (tc, td) = totals(&txns);
                out.push(AccountView {
                    label: format!("{:?}", a.account_type()),
                    account_id: mask(a.account_id().as_str()),
                    account_id_full: a.account_id().as_str().into(),
                    bank_id: Some(a.bank_id().as_str().into()),
                    branch_id: a.branch_id().map(|b| b.as_str().to_string()),
                    account_key: a.account_key().map(str::to_string),
                    fi_org: fi_org.clone(),
                    fi_id: fi_id.clone(),
                    currency: s.currency_default().as_str().into(),
                    server_date: server_date.clone(),
                    language: language.clone(),
                    ofx_version: ofx_version.clone(),
                    txn_uid: Some(w.transaction_uid().to_string()),
                    ledger: s.ledger_balance().map(|b| BalanceInfo {
                        display: fmt_amt(b.amount()),
                        as_of: fmt_dt(b.as_of()),
                        raw: b.amount().as_decimal(),
                    }),
                    available: s.available_balance().map(|b| BalanceInfo {
                        display: fmt_amt(b.amount()),
                        as_of: fmt_dt(b.as_of()),
                        raw: b.amount().as_decimal(),
                    }),
                    balance_list: extract_balance_list(s.balance_list()),
                    date_range: tl.map(|t| (fmt_dt(t.start()), fmt_dt(t.end()))),
                    transactions: txns,
                    total_credit: tc,
                    total_debit: td,
                });
            }
        }
    }

    if let Some(cc) = doc.credit_card() {
        for w in cc.statement_responses() {
            if let Some(s) = w.response() {
                let a = s.credit_card_account();
                let tl = s.transaction_list();
                let txns = extract_txns(tl);
                let (tc, td) = totals(&txns);
                out.push(AccountView {
                    label: "Credit Card".into(),
                    account_id: mask(a.account_id().as_str()),
                    account_id_full: a.account_id().as_str().into(),
                    bank_id: None,
                    branch_id: None,
                    account_key: a.account_key().map(str::to_string),
                    fi_org: fi_org.clone(),
                    fi_id: fi_id.clone(),
                    currency: s.currency_default().as_str().into(),
                    server_date: server_date.clone(),
                    language: language.clone(),
                    ofx_version: ofx_version.clone(),
                    txn_uid: Some(w.transaction_uid().to_string()),
                    ledger: s.ledger_balance().map(|b| BalanceInfo {
                        display: fmt_amt(b.amount()),
                        as_of: fmt_dt(b.as_of()),
                        raw: b.amount().as_decimal(),
                    }),
                    available: s.available_balance().map(|b| BalanceInfo {
                        display: fmt_amt(b.amount()),
                        as_of: fmt_dt(b.as_of()),
                        raw: b.amount().as_decimal(),
                    }),
                    balance_list: extract_balance_list(s.balance_list()),
                    date_range: tl.map(|t| (fmt_dt(t.start()), fmt_dt(t.end()))),
                    transactions: txns,
                    total_credit: tc,
                    total_debit: td,
                });
            }
        }
    }

    if let Some(inv) = doc.investment() {
        for w in inv.statement_responses() {
            if let Some(s) = w.response() {
                let a = s.investment_account();
                let tl = s.transaction_list();
                let txns = extract_txns(tl);
                let (tc, td) = totals(&txns);
                out.push(AccountView {
                    label: "Investment".into(),
                    account_id: mask(a.account_id().as_str()),
                    account_id_full: a.account_id().as_str().into(),
                    bank_id: Some(a.broker_id().into()),
                    branch_id: None,
                    account_key: None,
                    fi_org: fi_org.clone(),
                    fi_id: fi_id.clone(),
                    currency: s.currency_default().as_str().into(),
                    server_date: server_date.clone(),
                    language: language.clone(),
                    ofx_version: ofx_version.clone(),
                    txn_uid: Some(w.transaction_uid().to_string()),
                    ledger: None,
                    available: None,
                    balance_list: vec![],
                    date_range: tl.map(|t| (fmt_dt(t.start()), fmt_dt(t.end()))),
                    transactions: txns,
                    total_credit: tc,
                    total_debit: td,
                });
            }
        }
    }

    out
}

fn extract_balance_list(balances: &[ofx_rs::aggregates::Balance]) -> Vec<BalanceItem> {
    balances
        .iter()
        .map(|b| BalanceItem {
            name: b.name().to_string(),
            description: b.description().to_string(),
            kind: format!("{:?}", b.kind()),
            value: fmt_amt(b.value()),
            as_of: b.as_of().map(fmt_dt),
            currency: b.currency().map(|c| c.as_str().to_string()),
        })
        .collect()
}

fn extract_txns(tl: Option<&ofx_rs::aggregates::TransactionList>) -> Vec<TxnRow> {
    let Some(tl) = tl else {
        return vec![];
    };
    tl.transactions()
        .iter()
        .map(|t| {
            let raw = t.amount().as_decimal();
            TxnRow {
                date: fmt_dt(t.date_posted()),
                txn_type: fmt_txn_type(t.transaction_type()),
                name: t.name().unwrap_or("—").into(),
                memo: t.memo().unwrap_or("").into(),
                amount_str: fmt_amt(t.amount()),
                raw_amount: raw,
                fit_id: t.fit_id().as_str().to_string(),
                server_txn_id: t.server_transaction_id().map(|s| s.as_str().to_string()),
                check_number: t.check_number().map(|c| c.as_str().to_string()),
                reference_number: t.reference_number().map(str::to_string),
                payee_id: t.payee_id().map(str::to_string),
                sic: t.sic().map(|s| s.to_string()),
                correction_id: t.correction_id().map(|c| c.as_str().to_string()),
                correction_action: t.correction_action().map(|a| format!("{a:?}")),
                currency_info: t.currency().map(|c| {
                    let kind = match c {
                        ofx_rs::aggregates::CurrencyInfo::Currency { .. } => "Currency",
                        ofx_rs::aggregates::CurrencyInfo::OrigCurrency { .. } => "OrigCurrency",
                        _ => "Unknown",
                    };
                    format!("{} {} (rate: {})", kind, c.code().as_str(), c.rate())
                }),
                date_user: t.date_user().map(fmt_dt),
                date_available: t.date_available().map(fmt_dt),
            }
        })
        .collect()
}

fn totals(txns: &[TxnRow]) -> (Decimal, Decimal) {
    let mut credit = Decimal::ZERO;
    let mut debit = Decimal::ZERO;
    for t in txns {
        if t.raw_amount.is_sign_negative() {
            debit += t.raw_amount;
        } else {
            credit += t.raw_amount;
        }
    }
    (credit, debit)
}
