use crate::{execute_request, Client};
use serde::Deserialize;
use snafu::ResultExt;
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
    // pub players: PlayerEmbedding,
    pub date: Option<String>,
    pub submitted: Option<String>,
    pub times: Times,
    pub system: RunSystem,
    pub splits: Option<Splits>,
    pub values: HashMap<String, String>,
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
    pub primary_t: u64, // TODO: idk what number type
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

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("Failed accessing the Run with ID '{}'.", id))]
    Api { id: String, source: crate::Error },
}

pub async fn by_id(client: &Client, run_id: String) -> Result<Run, Error> {
    execute_request(client, runs_url(&run_id))
        .await
        .with_context(|| Api { id: run_id })
}
