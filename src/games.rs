use crate::common::Id;
use crate::{
    categories::{self, Category},
    execute_paginated_request, execute_request, Client, Data, Error,
};
use arrayvec::ArrayString;
use futures_util::stream::Stream;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;

pub use crate::common::Names;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Game {
    pub id: Id,
    pub names: Names,
    pub abbreviation: Box<str>,
    pub weblink: Box<str>,
    pub released: u16,
    pub release_date: ArrayString<[u8; 10]>,
    pub assets: Assets,
    pub ruleset: Rules,
    pub platforms: Vec<Id>,
    pub variables: Option<Data<Vec<Variable>>>,
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
    pub uri: Box<str>,
    pub width: u32,
    pub height: u32,
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
    pub id: Id,
    pub name: Box<str>,
    pub category: Option<Id>,
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
    pub values: HashMap<Id, VariableValue>,
    pub default: Option<Id>,
}

#[derive(Debug, Deserialize)]
pub struct VariableValue {
    pub label: Box<str>,
    pub rules: Option<Box<str>>,
}

#[derive(Debug, Deserialize)]
pub struct GameHeader {
    pub id: Id,
    pub names: Names,
    pub abbreviation: Box<str>,
    pub weblink: Box<str>,
}

impl Game {
    pub fn search<'client>(
        client: &'client Client,
        name: &str,
    ) -> impl Stream<Item = Result<Game, Error>> + 'client {
        self::search(client, name)
    }

    pub async fn categories(&self, client: &Client) -> Result<Vec<Category>, Error> {
        categories::for_game(client, &self.id).await
    }
}

impl GameHeader {
    pub async fn game(&self, client: &Client) -> Result<Game, Error> {
        by_id(client, &self.id).await
    }
}

pub fn all(
    client: &Client,
    elements_per_page: Option<u16>,
) -> impl Stream<Item = Result<GameHeader, Error>> + '_ {
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

    execute_paginated_request(client, url)
}

pub fn search<'client>(
    client: &'client Client,
    name: &str,
) -> impl Stream<Item = Result<Game, Error>> + 'client {
    let mut url = api_url!(games);
    url.query_pairs_mut().append_pair("name", name);

    execute_paginated_request(client, url)
}

pub async fn by_id(client: &Client, game_id: &str) -> Result<Game, Error> {
    let mut url = api_url!(games);
    url.path_segments_mut().unwrap().push(game_id);

    execute_request(client, url).await
}
