// TODO: Move this to players.rs
use crate::common::Id;
pub use crate::leaderboards::{Guest, Player};
use crate::{execute_request, Client, Error};
use arrayvec::ArrayString;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Run {
    pub id: Id,
    pub weblink: Box<str>,
    pub game: Id,
    pub category: Id,
    pub videos: Option<Videos>,
    pub comment: Option<Box<str>>,
    pub players: Players,
    pub date: Option<ArrayString<[u8; 10]>>,
    pub submitted: Option<ArrayString<[u8; 20]>>,
    pub times: Times,
    pub system: RunSystem,
    pub splits: Option<Splits>,
    pub values: HashMap<Id, Id>,
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
    pub id: Id,
}

#[derive(Debug, Deserialize)]
pub struct RunSystem {
    pub emulated: bool,
    pub platform: Id,
    pub region: Option<Id>,
}

#[derive(Debug, Deserialize)]
pub struct Videos {
    pub links: Option<Vec<Video>>,
}

#[derive(Debug, Deserialize)]
pub struct Video {
    pub uri: Box<str>,
}

#[derive(Debug, Deserialize)]
pub struct Times {
    pub primary: Box<str>,
    pub primary_t: f64,
}

#[derive(Debug, Deserialize)]
pub struct Splits {
    pub uri: Box<str>,
}

fn runs_url(run_id: &str) -> Url {
    let mut url = api_url!(runs);
    url.path_segments_mut().unwrap().push(run_id);
    url
}

pub async fn by_id(client: &Client, run_id: &str) -> Result<Run, Error> {
    execute_request(client, runs_url(run_id)).await
}
