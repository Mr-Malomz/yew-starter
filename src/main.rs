use yew::prelude::*;
mod components;
mod models;

use components::header::Header;

#[function_component(App)]
fn app() -> Html {
    html! {
      <>
        <Header />
      </>
    }
}

fn main() {
    yew::start_app::<App>();
}
