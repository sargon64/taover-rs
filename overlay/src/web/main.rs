use crate::ta::Model;
use yew::prelude::*;

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
                <p>{"uwu"}</p>
                <Model />// ws_uri={ctx.props().ws_uri.clone()}/>
            </>
        }
    }
}
