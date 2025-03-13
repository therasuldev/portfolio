pub mod app;
pub mod components;
pub mod data;
pub mod services;
pub mod routes;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
