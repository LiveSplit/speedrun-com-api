use futures_util::stream::{self, Stream};
use http::{Request, StatusCode};
use platform::Body;
use serde::{de::DeserializeOwned, Deserialize};
use snafu::ResultExt;
use url::Url;

macro_rules! api_url {
    ($resource:ident) => {
        url::Url::parse(concat!(
            "https://www.speedrun.com/api/v1/",
            stringify!($resource)
        ))
        .unwrap()
    };
}

mod platform;

pub mod categories;
pub mod common;
pub mod games;
pub mod leaderboards;
pub mod platforms;
pub mod regions;
pub mod runs;

pub use {
    categories::Category, games::Game, leaderboards::Leaderboard, platforms::Platform,
    regions::Region, runs::Run,
};

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed receiving the response from speedrun.com.
    Response { source: platform::Error },
    #[snafu(display("HTTP Status Code: {}", status.canonical_reason().unwrap_or_else(|| status.as_str())))]
    Status { status: StatusCode },
    #[snafu(display("{}", message))]
    Api {
        status: StatusCode,
        message: Box<str>,
    },
    /// Failed parsing the response from speedrun.com.
    Json { source: serde_json::Error },
}

#[derive(Deserialize)]
struct ApiError {
    message: Box<str>,
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
#[serde(tag = "rel")]
enum PaginationLink {
    #[serde(rename = "next")]
    Next { uri: Box<str> },
    #[serde(rename = "prev")]
    Previous { uri: Box<str> },
}

pub use platform::Client;

async fn execute_request_without_data<T: DeserializeOwned>(
    client: &Client,
    url: Url,
) -> Result<T, Error> {
    let response = client
        .request(Request::get(url.as_str()).body(Body::empty()).unwrap())
        .await
        .context(Response)?;
    let status = response.status();

    if !status.is_success() {
        if let Ok(reader) = platform::recv_reader(response.into_body()).await {
            if let Ok(error) = serde_json::from_reader::<_, ApiError>(reader) {
                return Err(Error::Api {
                    status,
                    message: error.message,
                });
            }
        }
        return Err(Error::Status { status });
    }

    let reader = platform::recv_reader(response.into_body())
        .await
        .context(Response)?;
    serde_json::from_reader(reader).context(Json)
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
                                        .find_map(|l| {
                                            if let PaginationLink::Next { uri } = l {
                                                Some(uri)
                                            } else {
                                                None
                                            }
                                        })
                                        .and_then(|uri| Url::parse(&uri).ok()),
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
