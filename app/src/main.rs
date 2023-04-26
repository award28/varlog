mod app;
mod hooks;
mod components;
mod organisms;
mod molecules;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
