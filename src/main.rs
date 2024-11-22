// mod app;

// use app::*;
// use leptos::{logging, mount};

// pub fn main() {
//     console_error_panic_hook::set_once();
//     logging::log!("csr mode - mounting to body");
//     mount::mount_to_body(App);
// }
mod app;
mod  portal_provider;
mod navigation;
mod bouncy_balls;
mod bouncy_squares;
use app::*;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App /> }
    });
}