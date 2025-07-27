mod app;
mod navigation;
mod portal_provider;
mod bouncy_balls;
mod bouncy_squares;

use app::*;
use leptos::{logging, mount};

pub fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");
    mount::mount_to_body(App);
}
