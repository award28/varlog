use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="d-flex align-items-center pb-3 mb-5 border-bottom">
            <a href="#" class="d-flex align-items-center text-body-emphasis text-decoration-none">
            <span class="fs-4">{ "Varlog" }</span>
            </a>
        </header>
    }
}
