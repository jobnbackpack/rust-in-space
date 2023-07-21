use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};

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

async fn fetch_launches() -> Result<Vec<Launch>> {
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
fn LaunchList(cx: Scope) -> impl IntoView {
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
