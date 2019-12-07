// TODO: Move this to players.rs
pub use crate::leaderboards::{Guest, Player};
use crate::{execute_request, Client, Error};
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Run {
    pub id: String,
    pub weblink: String,
    pub game: String,
    pub category: String,
    pub videos: Option<Videos>,
    pub comment: Option<String>,
    pub players: Players,
    pub date: Option<String>,
    pub submitted: Option<String>,
    pub times: Times,
    pub system: RunSystem,
    pub splits: Option<Splits>,
    pub values: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Players {
    Refs(Vec<PlayerRef>),
    Embedded { data: Vec<Player> },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "rel")]
#[serde(rename_all = "kebab-case")]
pub enum PlayerRef {
    User(UserRef),
    Guest(Guest),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserRef {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct RunSystem {
    pub emulated: bool,
    pub platform: String,
    pub region: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Videos {
    pub links: Option<Vec<Video>>,
}

#[derive(Debug, Deserialize)]
pub struct Video {
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Times {
    pub primary: String,
    pub primary_t: f64,
}

#[derive(Debug, Deserialize)]
pub struct Splits {
    pub uri: String,
}

fn runs_url(run_id: &str) -> Url {
    let mut url = api_url!(runs);
    url.path_segments_mut().unwrap().push(run_id);
    url
}

pub async fn by_id(client: &Client, run_id: String) -> Result<Run, Error> {
    execute_request(client, runs_url(&run_id)).await
}
