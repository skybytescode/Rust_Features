mod app;
mod content;
mod highlight;
mod i18n;
mod markdown;
mod search;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
