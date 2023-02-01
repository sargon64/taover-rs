use std::collections::VecDeque;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU8, Ordering};
use std::time::Duration;

use anyhow::Error;
use js_sys::Uint8Array;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, FileReader, MessageEvent, ProgressEvent, WebSocket};
use yew::{html, Callback, Component, Context, Html};
use yew_websocket::macros::Json;
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use ipc_channel::ipc;
use prost::Message;

use crate::console_log;
use crate::proto::packet::Packet;

// use crate::SENT_PACKET_COUNT;

// pub fn start_ta(ta_uri: &str) -> Result<(), JsValue> {
//     let ws = WebSocket::new(ta_uri)?;
//     let on_message = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
//         let blob = e.data().dyn_into::<web_sys::Blob>().unwrap();
//         let fr = FileReader::new().unwrap();
//         let fr_c = fr.clone();
//
//         let onloadend = Closure::<dyn FnMut(_)>::new(move |_e: ProgressEvent| {
//             let arr = Uint8Array::new(&fr_c.result().unwrap()).to_vec();
//             let mut reader = BytesReader::from_bytes(&arr);
//             let decoded = Packet::from_reader(&mut reader, &arr);
//             SENT_PACKET_COUNT.fetch_add(1, Ordering::SeqCst);
//             console_log!("{:?}", SENT_PACKET_COUNT.load(Ordering::SeqCst));
//         });
//         fr.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
//         fr.read_as_array_buffer(&blob).expect("blob not readable");
//         onloadend.forget();
//     });
//     ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
//     on_message.forget();
//     Ok(())
// }

type AsBinary = bool;

pub enum Format {
    Json,
    Toml,
}

pub enum WsAction {
    Connect,
    //SendData(AsBinary),
    Disconnect,
    Lost,
}

pub enum Msg {
    WsAction(WsAction),
    WsReady(Result<Vec<u8>, Error>),
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

pub struct Model {
    pub fetching: bool,
    pub data: Option<Packet>,
    pub ws: Option<WebSocketTask>,
}

static FAILURE_RETRY_COUNT: AtomicU8 = AtomicU8::new(0);

impl Model {
    fn view_data(&self) -> Html {
        if let Some(value) = &self.data {
            html! {
                <p>{ format!("{:?}", value) }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            fetching: false,
            data: None,
            ws: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = ctx.link().callback(|data| Msg::WsReady(data));
                    let notification = ctx.link().batch_callback(|status| match status {
                        WebSocketStatus::Opened => {
                            FAILURE_RETRY_COUNT.store(0, SeqCst);
                            None
                        }
                        WebSocketStatus::Closed => Some(WsAction::Lost.into()),
                        WebSocketStatus::Error => {
                            let mut res = WsAction::Connect.into();
                            if FAILURE_RETRY_COUNT.load(SeqCst) > 3 {
                                res = WsAction::Lost.into()
                            }
                            FAILURE_RETRY_COUNT.fetch_add(1, SeqCst);
                            console_log!("Waiting 5 seconds before retry on websocket error.");
                            let i = wasm_timer::Instant::now();
                            loop {
                                if wasm_timer::Instant::now().duration_since(i).as_secs() >= 5 {
                                    break;
                                }
                            }
                            Some(res)
                        }
                    });
                    let task = WebSocketService::connect_binary(
                        "ws://127.0.0.1:2053",
                        callback,
                        notification,
                    )
                    .unwrap();
                    self.ws = Some(task);
                    true
                }
                // WsAction::SendData(binary) => {
                //     let request = WsRequest { value: 321 };
                //     if binary {
                //         self.ws.as_mut().unwrap().send_binary(Json(&request));
                //     } else {
                //         self.ws.as_mut().unwrap().send(Json(&request));
                //     }
                //     false
                // }
                WsAction::Disconnect => {
                    self.ws.take();
                    true
                }
                WsAction::Lost => {
                    self.ws = None;
                    true
                }
            },
            Msg::WsReady(response) => {
                //self.data = self.data.ok();
                // let arr = response.unwrap();
                // let mut reader = BytesReader::from_bytes(&arr);
                // let decoded = Packet::from_reader(&mut reader, &arr);
                let deq = VecDeque::from(response.unwrap());
                self.data = Packet::decode(deq).ok();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <nav class="menu">
                    { self.view_data() }
                    <button disabled={self.ws.is_some()}
                            onclick={ctx.link().callback(|_| WsAction::Connect)}>
                        { "Connect To WebSocket" }
                    </button>
                    // <button disabled={self.ws.is_none()}
                    //         onclick={ctx.link().callback(|_| WsAction::SendData(false))}>
                    //     { "Send To WebSocket" }
                    // </button>
                    // <button disabled={self.ws.is_none()}
                    //         onclick={ctx.link().callback(|_| WsAction::SendData(true))}>
                    //     { "Send To WebSocket [binary]" }
                    // </button>
                    <button disabled={self.ws.is_none()}
                            onclick={ctx.link().callback(|_| WsAction::Disconnect)}>
                        { "Close WebSocket connection" }
                    </button>
                </nav>
            </div>
        }
    }
}
