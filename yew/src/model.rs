use yew::prelude::*;
use gloo::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Promise, Error};

#[wasm_bindgen(module = "/js/test.js")]
extern "C" {
    fn openFilePicker();
}
pub enum Msg {
    AddOne,
    FileOpen,
}
pub struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                console::log!("plus one");
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::FileOpen => {
                console::log!("file open");
                openFilePicker();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <h1>{"This file is from yew serve!"}</h1>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
                // <button onclick={link.callback(|_| Msg::FileOpen)}>{"only when https"}</button>
                <button onclick={link.callback(|_| Msg::FileOpen)}>{"only when https"}</button>
                // <button 'onclick=""' "[fileHandle] = await window.showOpenFilePicker(); console.log(fileHandle);">{ "only https" }</button>
            </div>
        }
    }
}
