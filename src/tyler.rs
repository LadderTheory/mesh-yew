use yew::{Component,Context,html,Html,Properties};

pub struct Tyler;

impl Component for Tyler {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Tyler
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            "Hi Tyler"
        }
    }
}