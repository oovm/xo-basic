use crate::{
    agents::global::{GlobalStore, Request},
    components::TextInput,
};
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
    inner: Game,
    text: Option<String>,
    store: Box<dyn Bridge<StoreWrapper<GlobalStore>>>,
}

impl Component for ShowBoard {
    type Message = Event;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Event::PostStoreMsg);
        Self { link, inner: Default::default(), text: None, store: GlobalStore::bridge(callback) }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::UpdateText(text) => {
                self.store.send(Request::UpdatePost(self.id, text));
                false
            }
            Event::Delete => {
                self.store.send(Request::RemovePost(self.id));
                false
            }
            Event::PostStoreMsg(state) => {
                let state = state.borrow();

                // Only update if the post changed.
                if let Some(text) = state.posts.get(&self.id) { self.text.neq_assign(Some(text.clone())) } else { false }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id.neq_assign(props.id)
    }

    fn view(&self) -> Html {
        let text = self.text.as_deref().unwrap_or("<pending>");

        html! {
            <div>
                <h2>{ format!("Post #{}", self.id) }</h2>
                <p>{text}</p>

                <TextInput value=text onsubmit=self.link.callback(Event::UpdateText) />
                <button onclick=self.link.callback(|_| Event::Delete)>
                    { "Delete" }
                </button>
            </div>
        }
    }
}
