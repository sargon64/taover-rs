use crate::console_log;

use super::proto::packet::Packet;

use js_sys::Uint8Array;
use quick_protobuf::{BytesReader, MessageRead};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebSocket, MessageEvent, FileReader, ProgressEvent, console};
// use ws::{Factory, Handler, Sender, WebSocket};

// pub struct TaConnection {
//     pub ta_uri: String,
//     pub ws: WebSocket<TaFactory>,
// }

// pub struct TaHandler {
//     pub ws: Sender,
//     pub is_client: bool,
// }

// impl Handler for TaHandler {
//     fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
//         let b = msg.into_data();
//         //println!("{:x?}", &b);
//         let mut reader = BytesReader::from_bytes(&b);
//         let decoded = Packet::from_reader(&mut reader, &b);
//         println!("{:?}", serde_json::to_string(&decoded.unwrap()));
//         ws::Result::Ok(())
//     }
// }

// pub struct TaFactory;

// impl Factory for TaFactory {
//     type Handler = TaHandler;

//     fn connection_made(&mut self, ws: ws::Sender) -> Self::Handler {
//         Self::Handler {
//             ws,
//             is_client: true,
//         }
//     }
// }

// impl TaConnection {
//     pub fn new(ta_uri: &str) -> Self {
//         let ws = WebSocket::new(TaFactory).unwrap();
//         Self {
//             ws,
//             ta_uri: ta_uri.to_owned(),
//         }
//     }
// }

pub fn start_ta(ta_uri: &str) -> Result<(), JsValue>{
    let ws = WebSocket::new(ta_uri)?;
    let on_message = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        let blob = e.data().dyn_into::<web_sys::Blob>().unwrap();
        let fr = FileReader::new().unwrap();
        let fr_c = fr.clone();

        let onloadend = Closure::<dyn FnMut(_)>::new(move |_e: ProgressEvent| {
            let arr = Uint8Array::new(&fr_c.result().unwrap()).to_vec();
            let mut reader = BytesReader::from_bytes(&arr);
            let decoded = Packet::from_reader(&mut reader, &arr);
            console_log!("{:?}", decoded)
        });
        fr.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
        fr.read_as_array_buffer(&blob).expect("blob not readable");
        onloadend.forget();
    });
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
    Ok(())
}