use crate::{common::Id, execute_paginated_request, execute_request, Client, Error};
use arrayvec::ArrayString;
use futures_util::stream::Stream;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Platform {
    pub id: Id,
    pub name: Box<str>,
}

pub fn all(
    client: &Client,
    elements_per_page: Option<u16>,
) -> impl Stream<Item = Result<Platform, Error>> + '_ {
    let mut url = api_url!(platforms);
    if let Some(elements) = elements_per_page {
        let mut buf = ArrayString::<[u8; 5]>::new();
        write!(buf, "{}", elements).unwrap();
        url.query_pairs_mut().append_pair("max", &buf);
    }

    execute_paginated_request(client, url)
}

pub async fn by_id(client: &Client, platform_id: &str) -> Result<Platform, Error> {
    let mut url = api_url!(platforms);
    url.path_segments_mut().unwrap().push(platform_id);

    execute_request(client, url).await
}
