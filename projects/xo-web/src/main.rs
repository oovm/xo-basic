#![recursion_limit = "1024"]
mod agents;
mod components;
mod localization;

use agents::global::{GlobalStore, Request};
use components::ShowBoard;
use yew::{prelude::*, services::ConsoleService};
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

pub enum Event {
    CreatePost(String),
    PostStoreMsg(ReadOnly<GlobalStore>),
}

pub struct Model {
    link: ComponentLink<Self>,
    store: Box<dyn Bridge<StoreWrapper<GlobalStore>>>,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Event::PostStoreMsg);

        Self { link, store: GlobalStore::bridge(callback) }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        // https://playtictactoe.org/
        let title = "Tic-Tac-Toe";
        let desc =
            "Play a retro version of tic-tac-toe (noughts and crosses, tres en raya) against the computer or with two players.";
        let player = "玩家";
        let bot = "电脑";
        let player1 = "玩家 1";
        let player2 = "玩家 2";
        let p_left = "(";
        let p_right = ")";
        let x_win = 0;
        let o_win = 0;
        let draw = 0;
        let h = "-";

        html! {
                        <>
        <span>{title}</span>
        <span>{desc}</span>
        <ShowBoard/>
        <div class="scores p1">
            <p class="player1"><span class="p1">{player}</span><span class="p2">{player1}</span> {p_left}<span
                    class="x"></span>{p_right}<span class="score">{x_win}</span></p>
            <p class="ties">{h}<span class="score">{draw}</span></p>
            <p class="player2"><span class="p1">{bot}</span><span class="p2">{player2}</span> {p_left}<span
                    class="o"></span>{p_right}<span class="score">{o_win}</span></p>
        </div>

                        </>
                    }
    }
}

fn main() {
    yew::start_app::<Model>();
}
