use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Names {
    pub international: Box<str>,
    pub japanese: Option<Box<str>>,
    pub twitch: Option<Box<str>>,
}
