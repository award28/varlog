mod app;
mod myapp;
mod videos;


fn main() {
    yew::Renderer::<myapp::App>::new().render();
}
