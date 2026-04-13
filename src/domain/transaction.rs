use rust_decimal::Decimal;

pub struct TxnRow {
    pub date: String,
    pub txn_type: String,
    pub name: String,
    pub memo: String,
    pub amount_str: String,
    pub raw_amount: Decimal,
    pub fit_id: String,
    pub server_txn_id: Option<String>,
    pub check_number: Option<String>,
    pub reference_number: Option<String>,
    pub payee_id: Option<String>,
    pub sic: Option<String>,
    pub correction_id: Option<String>,
    pub correction_action: Option<String>,
    pub currency_info: Option<String>,
    pub date_user: Option<String>,
    pub date_available: Option<String>,
}
