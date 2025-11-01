mod app;

use leptos::mount;
use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}
