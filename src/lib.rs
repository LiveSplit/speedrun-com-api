#![feature(async_await)]

use futures::stream;
use futures::Stream;
use futures::TryStreamExt;
use hyper_rustls::HttpsConnector;
use serde::{de::DeserializeOwned, Deserialize};
use snafu::ResultExt;
use url::Url;

macro_rules! api_url {
    ($resource:ident) => {
        Url::parse(concat!(
            "https://www.speedrun.com/api/v1/",
            stringify!($resource)
        ))
        .unwrap()
    };
}

pub mod games;
pub mod runs;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed receiving the response from speedrun.com.
    Response { source: hyper::Error },
    #[snafu(display("Unsuccessful Request: {}.", status))]
    Status { status: hyper::StatusCode },
    /// Failed parsing the response from speedrun.com.
    Json { source: serde_json::Error },
}

#[repr(transparent)]
#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
struct Page<T> {
    data: Vec<T>,
    pagination: Pagination,
}

#[derive(Debug, Deserialize)]
struct Pagination {
    offset: u64,
    max: u64,
    size: u64,
    links: Vec<PaginationLink>,
}

#[derive(Debug, Deserialize)]
struct PaginationLink {
    rel: PaginationRel,
    uri: String,
}

#[derive(Debug, Deserialize, PartialEq)]
enum PaginationRel {
    #[serde(rename = "next")]
    Next,
    #[serde(rename = "prev")]
    Previous,
}

pub type Client = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

async fn execute_request_without_data<T: DeserializeOwned>(
    client: &Client,
    url: Url,
) -> Result<T, Error> {
    let response = client
        .get(url.as_str().parse().unwrap())
        .await
        .context(Response)?;

    if !response.status().is_success() {
        return Err(Error::Status {
            status: response.status(),
        });
    }

    let buf = response.into_body().try_concat().await.context(Response)?;
    serde_json::from_reader(&*buf).context(Json)
}

async fn execute_request<T: DeserializeOwned>(client: &Client, url: Url) -> Result<T, Error> {
    let data: Data<T> = execute_request_without_data(client, url).await?;
    Ok(data.data)
}

fn execute_paginated_request<T: DeserializeOwned + 'static>(
    client: &Client,
    url: Url,
) -> impl Stream<Item = Result<T, Error>> + '_ {
    stream::unfold(
        (Vec::new().into_iter(), Some(url)),
        move |(mut remaining_elements, url)| {
            async move {
                Some(if let Some(element) = remaining_elements.next() {
                    (Ok(element), (remaining_elements, url))
                } else {
                    match execute_request_without_data::<Page<T>>(client, url?).await {
                        Ok(page) => {
                            let mut remaining_elements = page.data.into_iter();
                            (
                                Ok(remaining_elements.next()?),
                                (
                                    remaining_elements,
                                    page.pagination
                                        .links
                                        .into_iter()
                                        .find(|l| l.rel == PaginationRel::Next)
                                        .and_then(|l| Url::parse(&l.uri).ok()),
                                ),
                            )
                        }
                        Err(e) => (Err(e), (remaining_elements, None)),
                    }
                })
            }
        },
    )
}
