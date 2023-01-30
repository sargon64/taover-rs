#![allow(dead_code)]
#![cfg(target_arch = "wasm32")]

pub mod proto;
pub mod web;
pub mod ta;

use ta::start_ta;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use std::thread;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{ "Connect to TA at ws://127.0.0.1:2053" }</p>
            </div>
        }
    }
}



#[wasm_bindgen(start)]
pub fn run() {
    // let mut con = TaConnection::new("ws://127.0.0.1:2053");
    // con.ws
    //     .connect(url::Url::parse(&con.ta_uri).unwrap())
    //     .unwrap();
    
    // let ta_handle = thread::spawn(|| con.ws.run());
    start_ta("ws://127.0.0.1:2053");
    yew::Renderer::<App>::new().render();
}
