use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use thiserror::Error;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! {cx, <App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <p>"hello"</p>
        <LaunchList/>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchList {
    count: i32,
    previous: String,
    results: Vec<Launch>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchStatus {
    id: i64,
    name: String,
    abbrev: String,
    description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceProvider {
    id: i32,
    url: String,
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocketConfig {
    id: i32,
    url: String,
    name: String,
    family: String,
    full_name: String,
    variant: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rocket {
    id: i32,
    configuration: RocketConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Orbit {
    id: i32,
    name: String,
    abbrev: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mission {
    id: i32,
    name: String,
    description: String,
    launch_designator: String,
    #[serde(rename = "type")]
    type_field: String,
    orbit: Orbit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    id: i32,
    url: String,
    name: String,
    country_code: String,
    map_image: String,
    timezone_name: String,
    total_launch_count: i64,
    total_landing_count: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pad {
    id: i32,
    url: String,
    agency_id: i64,
    name: String,
    info_url: String,
    wiki_url: String,
    map_url: String,
    latitude: String,
    longitude: String,
    location: Location,
    country_code: String,
    map_image: String,
    total_launch_count: i64,
    orbital_launch_attempt_count: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    id: i32,
    url: String,
    name: String,
    description: String,
    agencies: Vec<Agency>,
    image_url: String,
    start_date: String,
    end_date: String,
    info_url: String,
    wiki_url: String,
    mission_patches: Vec<Patch>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patch {
    id: i32,
    name: String,
    priority: i64,
    image_url: String,
    agency: Agency,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agency {
    id: i32,
    url: String,
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Launch {
    id: String,
    url: String,
    slug: String,
    name: String,
    status: LaunchStatus,
    last_updated: String,
    net: String,
    window_end: String,
    window_start: String,
    net_precision: LaunchStatus,
    probability: i32,
    weather_concerns: String,
    holdreason: String,
    failreason: String,
    hashtag: String,
    launch_service_provider: ServiceProvider,
    rocket: Rocket,
    mission: Mission,
    pad: Pad,
    webcast_live: bool,
    image: String,
    infographic: String,
    program: Program,
    orbital_launch_attempt_count: i64,
    location_launch_attempt_count: i64,
    pad_launch_attempt_count: i64,
    agency_launch_attempt_count: i64,
    orbital_launch_attempt_count_year: i64,
    location_launch_attempt_count_year: i64,
    pad_launch_attempt_count_year: i64,
    agency_launch_attempt_count_year: i64,
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
    fetch_example(cx)
}
