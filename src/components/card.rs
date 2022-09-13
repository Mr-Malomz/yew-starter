use yew::prelude::*;

#[function_component(Card)]
pub fn card() -> Html {
    html! {
    <div class="m-3 p-4 border rounded d-flex align-items-center">
        <img src="https://robohash.org/hicveldicta.png?size=50x50&set=set1" class="mr-2" alt="img" />
        <div class="">
            <p class="fw-bold mb-1">{"Daniel Travolta"}</p>
            <p class="fw-normal mb-1">{"Male"}</p>
            <p class="fw-normal mb-1">{"atuny0@sohu.com"}</p>
            <p class="fw-normal mb-1">{"+63 791 675 8914"}</p>
        </div>
    </div>
    }
}
