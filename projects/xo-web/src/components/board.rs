use crate::agents::global::{GlobalStore, Request};
use xo_basic::Game;
use yew::prelude::*;
use yewtil::{
    store::{Bridgeable, ReadOnly, StoreWrapper},
    NeqAssign,
};

pub enum Event {
    UpdateText(String),
    Delete,
    PostStoreMsg(ReadOnly<GlobalStore>),
}

pub struct ShowBoard {
    link: ComponentLink<Self>,
    store: Box<dyn Bridge<StoreWrapper<GlobalStore>>>,
    inner: Game,
    text: Option<String>,
}

impl Component for ShowBoard {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Event::PostStoreMsg);
        Self { link, inner: Default::default(), text: None, store: GlobalStore::bridge(callback) }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
                <div class="game">
            <div class="board">
                <div class="square top left" tabindex="0">
                    <div class=""></div>
                </div>
                <div class="square top" tabindex="0">
                    <div class=""></div>
                </div>
                <div class="square top right" tabindex="0">
                    <div class=""></div>
                </div>
                <div class="square left" tabindex="0">
                    <div class="o"></div>
                </div>
                <div class="square" tabindex="0">
                    <div class="x"></div>
                </div>
                <div class="square right" tabindex="0">
                    <div class=""></div>
                </div>
                <div class="square bottom left" tabindex="0">
                    <div class=""></div>
                </div>
                <div class="square bottom" tabindex="0">
                    <div class=""></div>
                </div>
                <div class="square bottom right" tabindex="0">
                    <div class=""></div>
                </div>
            </div>
            <div class="restart" style="display: none;"></div>
        </div>
                }
    }
}
