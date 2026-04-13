use iced::widget::{column, row};
use iced::{Element, Fill, Task, Theme};
use ofx_rs::parse;

use crate::application::{build_accounts, pick_file};
use crate::domain::AccountView;
use crate::presentation::message::Msg;
use crate::presentation::views::{
    empty_state::view_empty, header::view_header, sidebar::view_sidebar,
    transaction_table::view_table,
};

pub struct App {
    file_name: Option<String>,
    accounts: Vec<AccountView>,
    tab: usize,
    expanded: Option<usize>,
    error: Option<String>,
}

impl App {
    pub fn new() -> (Self, Task<Msg>) {
        (
            Self {
                file_name: None,
                accounts: vec![],
                tab: 0,
                expanded: None,
                error: None,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        match &self.file_name {
            Some(n) => format!("OFX Viewer — {n}"),
            None => "OFX Viewer".into(),
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::Open => Task::perform(pick_file(), Msg::Loaded),
            Msg::Loaded(Some((name, content))) => {
                match parse(&content) {
                    Ok(doc) => {
                        self.accounts = build_accounts(&doc);
                        self.file_name = Some(name);
                        self.tab = 0;
                        self.expanded = None;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e.to_string());
                        self.accounts.clear();
                    }
                }
                Task::none()
            }
            Msg::Loaded(None) => Task::none(),
            Msg::Tab(i) => {
                self.tab = i;
                self.expanded = None;
                Task::none()
            }
            Msg::ToggleRow(i) => {
                self.expanded = if self.expanded == Some(i) {
                    None
                } else {
                    Some(i)
                };
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Msg> {
        let header = view_header(self.file_name.as_deref());

        let body: Element<Msg> = if self.accounts.is_empty() {
            view_empty(self.error.as_deref())
        } else {
            let idx = self.tab.min(self.accounts.len().saturating_sub(1));
            row![
                view_sidebar(&self.accounts, idx),
                view_table(&self.accounts[idx], self.expanded),
            ]
            .width(Fill)
            .height(Fill)
            .into()
        };

        column![header, body].height(Fill).into()
    }
}
