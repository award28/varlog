mod app;
mod myapp;
mod videos;
mod hooks;
mod components;
mod molecules;

fn main() {
    yew::Renderer::<myapp::App>::new().render();
}
