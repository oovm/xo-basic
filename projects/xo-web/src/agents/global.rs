use crate::localization::Localization;
use std::collections::HashMap;
use xo_basic::{Game, Player, Winner};
use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

#[derive(Debug)]
pub enum Request {
    ChangeLanguage(Localization),
    StartNewGame,
    EnableBot,
    DisableBot,
    GameFinish(Game),
    ReplayRecord,
    DownloadRecords,
}

#[derive(Debug)]
pub enum Action {
    ChangeLanguage(Localization),
    StartNewGame,
    ChangeBot(bool),
    GameFinish(Game),
    ReplayRecord,
    DownloadRecords,
}

pub struct GlobalStore {
    pub bot: bool,
    pub first_hand: Player,
    pub localization: Localization,
    pub score: (usize, usize, usize),
}

impl Store for GlobalStore {
    type Input = Request;
    type Action = Action;

    fn new() -> Self {
        Self { bot: true, first_hand: Default::default(), localization: Localization::English, score: (0, 0, 0) }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Request::ChangeLanguage(lang) => {
                link.send_message(Action::ChangeLanguage(lang));
            }
            Request::GameFinish(g) => {
                link.send_message(Action::GameFinish(g));
            }
            _ => unimplemented!(),
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::ChangeLanguage(lang) => self.localization = lang,
            _ => unimplemented!(),
        }
    }
}
