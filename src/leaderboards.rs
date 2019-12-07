use crate::runs::Run;
use crate::{execute_request, Client, Data, Error};
use arrayvec::ArrayString;
use serde::Deserialize;
use url::Url;

pub use crate::common::Names;

bitflags::bitflags! {
    #[derive(Default)]
    pub struct Embeds: u8 {
        const PLAYERS = 1 << 0;
    }
}

#[derive(Debug, Deserialize)]
pub struct Leaderboard {
    pub weblink: String,
    pub runs: Vec<Record>,
    pub players: Option<Data<Vec<Player>>>,
}

impl Leaderboard {
    // pub fn records_with_players(&self) -> impl Iterator<Item = (&Record, impl Iterator<Item = &Player>)> {
    //     let db = if let Some(players) = &self.players {
    //         &players.data[..]
    //     } else {
    //         &[]
    //     };
    //     self.runs.iter().map(|run| {
    //         match &run.players {

    //         }
    //         (run, unimplemented!())
    //     })
    // }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "rel")]
#[serde(rename_all = "kebab-case")]
pub enum Player {
    User(User),
    Guest(Guest),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct User {
    pub id: String,
    pub names: Names,
    pub weblink: String,
    pub name_style: NameStyle,
    pub location: Option<UserLocation>,
}

#[derive(Debug, Deserialize)]
pub struct Guest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "style")]
#[serde(rename_all = "kebab-case")]
pub enum NameStyle {
    #[serde(rename_all = "kebab-case")]
    Solid { color: Color },
    #[serde(rename_all = "kebab-case")]
    Gradient { color_from: Color, color_to: Color },
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub light: String,
    pub dark: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLocation {
    pub country: UserCountry,
}

#[derive(Debug, Deserialize)]
pub struct UserCountry {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub place: u32,
    pub run: Run,
}

pub async fn get(
    client: &Client,
    game_id: String,
    category_id: String,
    embeds: Embeds,
) -> Result<Leaderboard, Error> {
    let mut url = api_url!(leaderboards);
    url.path_segments_mut()
        .unwrap()
        .extend(&[&game_id, "category", &category_id]);

    if !embeds.is_empty() {
        let mut buf = ArrayString::<[u8; 8]>::new();
        for &(flag, name) in &[(Embeds::PLAYERS, "players")] {
            if embeds.contains(flag) {
                if !buf.is_empty() {
                    buf.push_str(",");
                }
                buf.push_str(name);
            }
        }
        url.query_pairs_mut().append_pair("embed", &buf);
    }

    execute_request(client, dbg!(url)).await
}
