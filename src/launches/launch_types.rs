use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchListDto {
    pub count: Option<i32>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub results: Vec<Launch>,
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
    pub id: Option<String>,
    pub url: Option<String>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub status: Option<LaunchStatus>,
    pub last_updated: Option<String>,
    pub net: Option<String>,
    pub window_end: Option<String>,
    pub window_start: Option<String>,
    pub net_precision: Option<LaunchStatus>,
    pub probability: Option<i32>,
    pub weather_concerns: Option<String>,
    pub holdreason: Option<String>,
    pub failreason: Option<String>,
    pub hashtag: Option<String>,
    pub launch_service_provider: Option<String>,
    pub rocket: Option<String>,
    pub mission: Option<String>,
    pub pad: Option<String>,
    pub image: Option<String>,
    pub infographic: Option<String>,
    pub program: Option<Program>,
    pub orbital_launch_attempt_count: Option<i64>,
    pub location_launch_attempt_count: Option<i64>,
    pub pad_launch_attempt_count: Option<i64>,
    pub agency_launch_attempt_count: Option<i64>,
    pub orbital_launch_attempt_count_year: Option<i64>,
    pub location_launch_attempt_count_year: Option<i64>,
    pub pad_launch_attempt_count_year: Option<i64>,
    pub agency_launch_attempt_count_year: Option<i64>,
}
