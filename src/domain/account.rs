use rust_decimal::Decimal;

use super::TxnRow;

pub struct BalanceInfo {
    pub display: String,
    pub as_of: String,
    pub raw: Decimal,
}

pub struct BalanceItem {
    pub name: String,
    pub description: String,
    pub kind: String,
    pub value: String,
    pub as_of: Option<String>,
    pub currency: Option<String>,
}

pub struct AccountView {
    pub label: String,
    pub account_id: String,
    pub account_id_full: String,
    pub bank_id: Option<String>,
    pub branch_id: Option<String>,
    pub account_key: Option<String>,
    pub fi_org: Option<String>,
    pub fi_id: Option<String>,
    pub currency: String,
    pub server_date: String,
    pub language: String,
    pub ofx_version: String,
    pub txn_uid: Option<String>,
    pub ledger: Option<BalanceInfo>,
    pub available: Option<BalanceInfo>,
    pub balance_list: Vec<BalanceItem>,
    pub date_range: Option<(String, String)>,
    pub transactions: Vec<TxnRow>,
    pub total_credit: Decimal,
    pub total_debit: Decimal,
}
