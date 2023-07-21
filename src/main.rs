use leptos::*;
use leptos_router::*;

use crate::launches::launches::LaunchList;

mod launches;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! {cx,
                    <App/>
        }
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <Router>
            <div class="container">
            <nav>
                <ul>
                    <li><strong>"Rust-In-Space"</strong></li>
                </ul>
                <ul>
                    <li><A href="/">Home</A></li>
                    <li><A href="/lol">Lol</A></li>
                </ul>
            </nav>
            <Routes>
                <Route path="/" view=LaunchList/>
                <Route path="/lol" view=TestView/>
            </Routes>
            </div>
        </Router>
    }
}

#[component]
fn TestView(cx: Scope) -> impl IntoView {
    view! {cx,
        <h3>lol</h3>
    }
}
