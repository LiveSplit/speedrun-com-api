use crate::common::Id;
use crate::{
    execute_request,
    games::Game,
    leaderboards::{self, Leaderboard},
    Client, Error,
};
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

impl Category {
    pub async fn leaderboard(
        &self,
        client: &Client,
        game: &Game,
        embeds: leaderboards::Embeds,
    ) -> Result<Leaderboard, Error> {
        leaderboards::get(client, &game.id, &self.id, embeds).await
    }
}

pub async fn for_game(client: &Client, game_id: &str) -> Result<Vec<Category>, Error> {
    let mut url = api_url!(games);
    url.path_segments_mut()
        .unwrap()
        .extend(&[game_id, "categories"]);

    execute_request(client, url).await
}

pub async fn by_id(client: &Client, category_id: &str) -> Result<Category, Error> {
    let mut url = api_url!(categories);
    url.path_segments_mut().unwrap().push(category_id);

    execute_request(client, url).await
}
