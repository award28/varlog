use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub title: String,
}

#[function_component(Header)]
pub fn header(HeaderProps{ title }: &HeaderProps) -> Html {
    html! {
        <header class="d-flex align-items-center pb-3 mb-5 border-bottom">
            <a href="/" class="d-flex align-items-center text-body-emphasis text-decoration-none">
            <span class="fs-4">{ title }</span>
            </a>
        </header>
    }
}
