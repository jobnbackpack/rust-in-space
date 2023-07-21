use super::launch_types::{Launch, LaunchListDto};
use leptos::{error::Result, *};

async fn fetch_launches() -> Result<Vec<Launch>> {
    // make the request
    let res = reqwasm::http::Request::get("https://lldev.thespacedevs.com/2.2.0/launch?mode=list")
        .send()
        .await?
        // convert it to JSON
        .json::<LaunchListDto>()
        .await?
        .results
        // extract the name field for each launch
        .into_iter()
        // .map(|launch| launch.name.unwrap_or(String::from("empty")))
        .collect::<Vec<_>>();
    Ok(res)
}

#[component]
fn LaunchCard(cx: Scope, launch: Launch) -> impl IntoView {
    view! {cx,
        <article>
            <header>{launch.name.unwrap_or(String::from("empty"))}</header>
            <p><strong>Mission: </strong>{launch.mission.unwrap_or(String::from("not known"))}</p>
            <img src={launch.image.unwrap()} />
        </article>
    }
}

#[component]
pub fn LaunchList(cx: Scope) -> impl IntoView {
    let launches = create_local_resource(cx, || (), |_| async move { fetch_launches().await });

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{e.to_string()}</li> })
                    .collect_view(cx)
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let launch_list_view = move || {
        launches.read(cx).map(|data| {
            data.map(|data| {
                data.iter()
                    .map(|launch| view! { cx, <LaunchCard launch={launch.to_owned()}/> })
                    .collect_view(cx)
            })
        })
    };

    view! { cx,
        <div>
            <ErrorBoundary fallback>
                <Transition fallback=move || {
                    view! { cx, <div>"Loading (Suspense Fallback)..."</div> }
                }>
                <div>
                    {launch_list_view}
                </div>
                </Transition>
            </ErrorBoundary>
        </div>
    }
}
