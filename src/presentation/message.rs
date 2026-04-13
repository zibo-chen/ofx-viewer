#[derive(Debug, Clone)]
pub enum Msg {
    Open,
    Loaded(Option<(String, String)>),
    Tab(usize),
    ToggleRow(usize),
}
