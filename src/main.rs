use yew::prelude::*;
//use tyler::Tyler;
use mesh_field::MeshField;

//mod tyler;
mod mesh_field;
mod particle;
mod math;
mod line;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <MeshField/>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
