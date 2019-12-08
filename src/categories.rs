use crate::common::Id;
use crate::{execute_request, Client, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: Id,
    pub weblink: Box<str>,
    pub name: Box<str>,
    #[serde(rename = "type")]
    pub kind: CategoryKind,
    pub rules: Option<Box<str>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryKind {
    PerGame,
    PerLevel,
}

pub async fn for_game(client: &Client, game_id: &str) -> Result<Vec<Category>, Error> {
    let mut url = api_url!(games);
    url.path_segments_mut()
        .unwrap()
        .extend(&[game_id, "categories"]);

    execute_request(client, url).await
}
