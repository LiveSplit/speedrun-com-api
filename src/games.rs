use crate::{execute_request, execute_paginated_request, Client, Data};
use arrayvec::ArrayString;
use futures::{Stream, StreamExt};
use serde::Deserialize;
use snafu::ResultExt;
use std::collections::HashMap;
use std::fmt::Write;
use std::mem;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Game {
    pub id: String,
    pub names: Names,
    pub abbreviation: String,
    pub weblink: String,
    pub released: u64, // TODO
    pub release_date: String,
    pub assets: Assets,
    pub ruleset: Rules,
    pub platforms: Vec<String>,
    pub variables: Option<Data<Vec<Variable>>>,
}

#[derive(Debug, Deserialize)]
pub struct Names {
    pub international: String,
    pub japanese: Option<String>,
    pub twitch: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Assets {
    pub logo: Asset,
    pub cover_tiny: Asset,
    pub cover_small: Asset,
    pub cover_medium: Asset,
    pub cover_large: Asset,
    pub icon: Asset,
    pub trophy_1st: Asset,
    pub trophy_2nd: Asset,
    pub trophy_3rd: Asset,
    pub trophy_4th: Option<Asset>,
    pub background: Option<Asset>,
    pub foreground: Option<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub uri: String,
    pub width: u64, // TODO: Number Type
    pub height: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rules {
    pub show_milliseconds: bool,
    pub require_verification: bool,
    pub require_video: bool,
    pub run_times: Vec<TimingMethod>,
    pub default_time: TimingMethod,
    pub emulators_allowed: bool,
}

#[derive(Debug, Deserialize)]
pub enum TimingMethod {
    #[serde(rename = "realtime")]
    RealTime,
    #[serde(rename = "realtime_noloads")]
    RealTimeNoLoads,
    #[serde(rename = "ingame")]
    InGame,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Variable {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub scope: VariableScope,
    pub values: VariableValues,
    pub mandatory: bool,
    pub is_subcategory: bool,
}

#[derive(Debug, Deserialize)]
pub struct VariableScope {
    #[serde(rename = "type")]
    pub kind: VariableScopeKind,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VariableScopeKind {
    Global,
    FullGame,
    AllLevels,
    SingleLevel,
}

#[derive(Debug, Deserialize)]
pub struct VariableValues {
    pub values: HashMap<String, VariableValue>,
    pub default: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VariableValue {
    pub label: String,
    pub rules: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GameHeader {
    pub id: String,
    pub names: Names,
    pub abbreviation: String,
    pub weblink: String,
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("Failed accessing the Game with ID '{}'.", id))]
    Api { id: String, source: crate::Error },
}

#[derive(Debug, snafu::Snafu)]
pub enum SearchError {
    #[snafu(display("Failed searching for the game '{}'.", name))]
    Search { name: String, source: crate::Error },
}

#[derive(Debug, snafu::Snafu)]
pub enum HeadersError {
    /// Failed enumerating all the games on speedrun.com.
    Headers { source: crate::Error },
}

pub fn all(
    client: &Client,
    elements_per_page: Option<u16>,
) -> impl Stream<Item = Result<GameHeader, HeadersError>> + '_ {
    let mut url = api_url!(games);
    let mut buf = ArrayString::<[u8; 5]>::new();
    let elements = if let Some(elements) = elements_per_page {
        write!(buf, "{}", elements).unwrap();
        &buf
    } else {
        "1000"
    };
    url.query_pairs_mut()
        .append_pair("_bulk", "yes")
        .append_pair("max", elements);

    execute_paginated_request(client, url).map(|item| item.context(Headers))
}

pub fn search(
    client: &Client,
    mut name: String,
) -> impl Stream<Item = Result<Game, SearchError>> + '_ {
    let mut url = api_url!(games);
    url.query_pairs_mut().append_pair("name", &name);

    execute_paginated_request(client, url).map(move |item| {
        item.with_context(|| Search {
            name: mem::replace(&mut name, String::new()),
        })
    })
}

pub async fn by_id(client: &Client, game_id: String) -> Result<Game, Error> {
    let mut url = api_url!(games);
    url.path_segments_mut().unwrap().push(&game_id);

    execute_request(client, url)
        .await
        .with_context(|| Api { id: game_id })
}
