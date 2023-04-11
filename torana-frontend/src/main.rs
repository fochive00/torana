mod components;
mod pages;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <pages::Home />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
