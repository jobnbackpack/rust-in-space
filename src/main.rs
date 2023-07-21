use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchList {
    count: Option<i32>,
    previous: Option<String>,
    next: Option<String>,
    results: Vec<Launch>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchStatus {
    id: Option<i64>,
    name: Option<String>,
    abbrev: Option<String>,
    description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceProvider {
    id: Option<i32>,
    url: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocketConfig {
    id: Option<i32>,
    url: Option<String>,
    name: Option<String>,
    family: Option<String>,
    full_name: Option<String>,
    variant: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rocket {
    id: Option<i32>,
    configuration: Option<RocketConfig>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Orbit {
    id: Option<i32>,
    name: Option<String>,
    abbrev: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mission {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    launch_designator: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<String>,
    orbit: Option<Orbit>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    id: Option<i32>,
    url: Option<String>,
    name: Option<String>,
    country_code: Option<String>,
    map_image: Option<String>,
    timezone_name: Option<String>,
    total_launch_count: Option<i64>,
    total_landing_count: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pad {
    id: Option<i32>,
    url: Option<String>,
    agency_id: Option<i64>,
    name: Option<String>,
    info_url: Option<String>,
    wiki_url: Option<String>,
    map_url: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    location: Option<Location>,
    country_code: Option<String>,
    map_image: Option<String>,
    total_launch_count: Option<i64>,
    orbital_launch_attempt_count: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    id: Option<i32>,
    url: Option<String>,
    name: Option<String>,
    description: Option<String>,
    agencies: Option<Vec<Agency>>,
    image_url: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    info_url: Option<String>,
    wiki_url: Option<String>,
    mission_patches: Option<Vec<Patch>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patch {
    id: Option<i32>,
    name: Option<String>,
    priority: Option<i64>,
    image_url: Option<String>,
    agency: Option<Agency>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agency {
    id: Option<i32>,
    url: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Launch {
    id: Option<String>,
    url: Option<String>,
    slug: Option<String>,
    name: Option<String>,
    status: Option<LaunchStatus>,
    last_updated: Option<String>,
    net: Option<String>,
    window_end: Option<String>,
    window_start: Option<String>,
    net_precision: Option<LaunchStatus>,
    probability: Option<i32>,
    weather_concerns: Option<String>,
    holdreason: Option<String>,
    failreason: Option<String>,
    hashtag: Option<String>,
    launch_service_provider: Option<String>,
    rocket: Option<String>,
    mission: Option<String>,
    pad: Option<String>,
    image: Option<String>,
    infographic: Option<String>,
    program: Option<Program>,
    orbital_launch_attempt_count: Option<i64>,
    location_launch_attempt_count: Option<i64>,
    pad_launch_attempt_count: Option<i64>,
    agency_launch_attempt_count: Option<i64>,
    orbital_launch_attempt_count_year: Option<i64>,
    location_launch_attempt_count_year: Option<i64>,
    pad_launch_attempt_count_year: Option<i64>,
    agency_launch_attempt_count_year: Option<i64>,
}

async fn fetch_launches() -> Result<Vec<String>> {
    // make the request
    let res = reqwasm::http::Request::get("https://lldev.thespacedevs.com/2.2.0/launch?mode=list")
        .send()
        .await?
        // convert it to JSON
        .json::<LaunchList>()
        .await?
        .results
        // extract the name field for each launch
        .into_iter()
        .map(|launch| launch.name.unwrap_or(String::from("empty")))
        .collect::<Vec<_>>();
    Ok(res)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}

#[derive(Error, Clone, Debug)]
pub enum CatError {
    #[error("Please request more than zero cats.")]
    NonZeroCats,
}

type CatCount = usize;

async fn fetch_cats(count: CatCount) -> Result<Vec<String>> {
    if count > 0 {
        // make the request
        let res = reqwasm::http::Request::get(&format!(
            "https://api.thecatapi.com/v1/images/search?limit={count}",
        ))
        .send()
        .await?
        // convert it to JSON
        .json::<Vec<Cat>>()
        .await?
        // extract the URL field for each cat
        .into_iter()
        .take(count)
        .map(|cat| cat.url)
        .collect::<Vec<_>>();
        Ok(res)
    } else {
        Err(CatError::NonZeroCats.into())
    }
}

pub fn fetch_example(cx: Scope) -> impl IntoView {
    let (cat_count, set_cat_count) = create_signal::<CatCount>(cx, 0);

    // we use local_resource here because
    // 1) our error type isn't serializable/deserializable
    // 2) we're not doing server-side rendering in this example anyway
    //    (during SSR, create_resource will begin loading on the server and resolve on the client)
    let cats = create_local_resource(cx, cat_count, fetch_cats);

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

    // the renderer can handle Option<_> and Result<_> states
    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just implement our happy path and let the framework handle the rest
    let cats_view = move || {
        cats.read(cx).map(|data| {
            data.map(|data| {
                data.iter()
                    .map(|s| view! { cx, <p><img src={s}/></p> })
                    .collect_view(cx)
            })
        })
    };

    view! { cx,
        <div>
            <label>
                "How many cats would you like?"
                <input
                    type="number"
                    prop:value=move || cat_count.get().to_string()
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<CatCount>().unwrap_or(0);
                        set_cat_count(val);
                    }
                />
            </label>
            <ErrorBoundary fallback>
                <Transition fallback=move || {
                    view! { cx, <div>"Loading (Suspense Fallback)..."</div> }
                }>
                <div>
                    {cats_view}
                </div>
                </Transition>
            </ErrorBoundary>
        </div>
    }
}

#[component]
fn LaunchList(cx: Scope) -> impl IntoView {
    // fetch_example(cx)

    // we use local_resource here because
    // 1) our error type isn't serializable/deserializable
    // 2) we're not doing server-side rendering in this example anyway
    //    (during SSR, create_resource will begin loading on the server and resolve on the client)
    let cats = create_local_resource(cx, || (), |_| async move { fetch_launches().await });

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

    // the renderer can handle Option<_> and Result<_> states
    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just implement our happy path and let the framework handle the rest
    let cats_view = move || {
        cats.read(cx).map(|data| {
            data.map(|data| {
                data.iter()
                    .map(|s| view! { cx, <p>{s}</p> })
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
                    {cats_view}
                </div>
                </Transition>
            </ErrorBoundary>
        </div>
    }
}
