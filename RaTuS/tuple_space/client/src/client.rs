use crate::error::Error;
use crate::result::Result;
use reqwest::{StatusCode, Url};
use ts_core::query_tuple::QueryTuple;
use ts_core::tuple::Tuple;

pub struct Client {
    size_url: Url,
    write_url: Url,
    read_url: Url,
    get_url: Url,
    http_client: reqwest::Client,
}

#[derive(Default)]
pub struct Builder {}

impl Client {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn size(&self) -> Result<usize> {
        let response = self.http_client.get(self.size_url.clone()).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<usize>().await?),
            _ => Err(Error::ServerError),
        }
    }

    pub async fn write(&self, tuple: &Tuple) -> Result<()> {
        let response = self
            .http_client
            .post(self.write_url.clone())
            .body(serde_json::to_string(tuple)?)
            .send()
            .await?;

        match response.status() {
            StatusCode::CREATED => Ok(()),
            _ => Err(Error::ServerError),
        }
    }

    pub async fn read(&self, tuple: &QueryTuple) -> Result<Option<Tuple>> {
        let response = self
            .http_client
            .post(self.read_url.clone())
            .body(serde_json::to_string(tuple)?)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(Some(response.json::<Tuple>().await?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(Error::ServerError),
        }
    }

    pub async fn get(&self, tuple: &QueryTuple) -> Result<Option<Tuple>> {
        let response = self
            .http_client
            .post(self.get_url.clone())
            .body(serde_json::to_string(tuple)?)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(Some(response.json::<Tuple>().await?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(Error::ServerError),
        }
    }
}

impl Builder {
    pub fn build(&self, server: &str) -> Result<Client> {
        let base_server = Url::parse(server)?;
        let size_url = base_server.join("size")?;
        let read_url = base_server.join("read")?;
        let get_url = base_server.join("get")?;
        let write_url = base_server.join("write")?;

        Ok(Client {
            http_client: reqwest::Client::new(),
            size_url,
            read_url,
            get_url,
            write_url,
        })
    }
}
