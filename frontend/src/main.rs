pub mod components;
pub mod utils;

use components::App;

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default())
}
