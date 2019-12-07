use crate::{execute_request, Client, Error};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
    pub weblink: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: CategoryKind,
    pub rules: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryKind {
    PerGame,
    PerLevel,
}

pub async fn for_game(client: &Client, game_id: String) -> Result<Vec<Category>, Error> {
    let mut url = api_url!(games);
    url.path_segments_mut()
        .unwrap()
        .extend(&[&game_id, "categories"]);

    execute_request(client, url).await
}
