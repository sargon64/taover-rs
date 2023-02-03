use yew::prelude::*;

use crate::ta::Socket;
use crate::web::starting::StartingPage;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                //<Model />// ws_uri={ctx.props().ws_uri.clone()}/>
                <StartingPage />
            </>
        }
    }
}
