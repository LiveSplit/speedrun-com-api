use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Names {
    pub international: String,
    pub japanese: Option<String>,
    pub twitch: Option<String>,
}
