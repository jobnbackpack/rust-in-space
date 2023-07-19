use leptos::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn load_data() -> User {
    let response =
        reqwest::get("https://api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers")
            .await
            .unwrap()
            .json::<Vec<User>>()
            .await;

    log!("{:?}", response);

    User {
        login: "test".to_string(),
        id: 22,
    }
    // match response {
    //     Ok(users) => {
    //         log!("Processing user response {:?}", users);
    //         return user.;
    //     }
    //     Err(e) => {
    //         error!("Orders API response cannot be parsed! {}", e)
    //     }
    // }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <p>"Hello"</p>
    }
}

#[component]
fn LaunchList(cx: Scope) -> impl IntoView {
    let once = create_resource(cx, || (), |_| async move { load_data() });
    view! { cx,
        <h1>"My Data"</h1>
        {move || match once.read(cx) {
            None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
            // Some(data) => view! { cx, <ShowData data/> }.into_view(cx)
             Some(data) => {
                log!("{:?}", data);
                view! { cx, <p>"Ready!"</p> }.into_view(cx)
            }
        }}
    }
}
