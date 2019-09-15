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
        unimplemented!()
        // Self { link, post_ids: Vec::new(), store: GlobalStore::bridge(callback) }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                </div>
            </>
        }
    }
}
fn main() {
    yew::start_app::<Model>();
}
