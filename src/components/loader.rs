use yew::prelude::*;

#[function_component(Loader)]
fn loader() -> Html {
    html! {
    <div class="spinner-border" role="status">
        <span class="visually-hidden">{"Loading..."}</span>
    </div>
    }
}
