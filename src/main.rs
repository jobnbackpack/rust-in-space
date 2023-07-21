use leptos::*;

use crate::launches::launches::LaunchList;

mod launches;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! {cx, <div class="container"><App/></div>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <h1>"Rust-In-Space"</h1>
        <LaunchList/>
    }
}
