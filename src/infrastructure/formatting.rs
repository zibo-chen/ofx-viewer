use rust_decimal::Decimal;

pub fn fmt_amt(a: ofx_rs::types::OfxAmount) -> String {
    let d = a.as_decimal();
    let abs = d.abs();
    let s = format!("{abs:.2}");
    let (integer, decimal) = s.split_once('.').unwrap_or((&s, "00"));
    let formatted = thousands(integer);
    if d.is_sign_negative() {
        format!("-${formatted}.{decimal}")
    } else {
        format!("${formatted}.{decimal}")
    }
}

pub fn fmt_dec(d: Decimal) -> String {
    let abs = d.abs();
    let s = format!("{abs:.2}");
    let (integer, decimal) = s.split_once('.').unwrap_or((&s, "00"));
    format!("${}.{decimal}", thousands(integer))
}

fn thousands(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

pub fn fmt_dt(dt: &ofx_rs::types::OfxDateTime) -> String {
    let o = dt.as_offset_date_time();
    format!("{:04}-{:02}-{:02}", o.year(), o.month() as u8, o.day())
}

pub fn fmt_txn_type(t: ofx_rs::types::TransactionType) -> String {
    use ofx_rs::types::TransactionType::*;
    match t {
        Credit => "Credit",
        Debit => "Debit",
        Interest => "Interest",
        Dividend => "Dividend",
        Fee => "Fee",
        ServiceCharge => "Svc Fee",
        Deposit => "Deposit",
        Atm => "ATM",
        Pos => "POS",
        Transfer => "Transfer",
        Check => "Check",
        Payment => "Payment",
        Cash => "Cash",
        DirectDeposit => "Dir Dep",
        DirectDebit => "Dir Dbt",
        RepeatPayment => "Repeat",
        Hold => "Hold",
        Other => "Other",
        _ => "Unknown",
    }
    .into()
}

pub fn mask(id: &str) -> String {
    if id.len() > 4 {
        format!("····{}", &id[id.len() - 4..])
    } else {
        id.into()
    }
}
