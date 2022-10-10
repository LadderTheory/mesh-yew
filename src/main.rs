use yew::prelude::*;
//use tyler::Tyler;
use mesh_field::MeshField;

//mod tyler;
mod mesh_field;
mod particle;
mod math;
mod line;

struct App {
    node_ref: NodeRef,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let svg = html! {
            <MeshField/>
        };

        svg
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
