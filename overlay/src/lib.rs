#![allow(dead_code)]
#![cfg(target_arch = "wasm32")]

use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::props;
use yew_router::prelude::*;

use proto::packet::Packet;
//use ta::start_ta;

pub mod proto;
pub mod ta;
pub mod web;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

// pub struct App;
//
// impl Component for App {
//     type Message = ();
//     type Properties = ();
//
//     fn create(ctx: &Context<Self>) -> Self {
//         Self
//     }
//
//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div>
//                 <p>{ "Connect to TA at ws://127.0.0.1:2053" }</p>
//             </div>
//         }
//     }
//}

// struct AppProps<'a> {
//     rx: ipc::IpcReceiver<Packet<'a>>,
// }

// #[derive(Properties, PartialEq)]
// pub struct AppProps<'a> {
//     pub rx: ipc::IpcReceiver<Packet<'a>>,
// }
// lazy_static! {
//     // pub static ref CHANNEL: (
//     //     Arc<ipc::IpcSender<Packet<'static>>>,
//     //     Arc<ipc::IpcReceiver<Packet<'static>>>
//     // ) = {
//     //     let (tx, rx) = ipc::channel().unwrap();
//     //     (Arc::new(tx), Arc::new(rx))
//     // };
//     pub static ref CHANNEL: (Mutex<std::sync::mpsc::Sender<Packet<'static>>>, Mutex<std::sync::mpsc::Receiver<Packet<'static>>>) = {
//         let (tx, rx) = std::sync::mpsc::channel();
//         (Mutex::new(tx), Mutex::new(rx))
//     };
//     pub static ref SENT_PACKET_COUNT: AtomicU32 = {
//         0.into()
//     };
// }

pub fn delay(dur: Duration) {
    let i = wasm_timer::Instant::now();
    loop {
        if wasm_timer::Instant::now().duration_since(i).as_secs() >= dur.as_secs() {
            break;
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    //let (tx, rx) = ipc::channel().unwrap();
    //start_ta("ws://127.0.0.1:2053");
    // yew::start_app_with_props::<web::main::App>(props! {
    //     // ModelProps {
    //     //     ws_uri: "ws://127.0.0.1:2023/"
    //     // }
    // });
    yew::start_app::<web::main::App>();
}
