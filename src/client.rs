use crate::errors::{Error, Result};
use crate::request::{Request, RequestBuilderExt};
use futures::prelude::*;
use reqwest::Client as ReqwestClient;
use std::borrow::Cow;
use std::env;
use std::sync::Arc;

/// The main client used for making request to IEX.
///
/// `IexConfig` stores an async Reqwest client as well as the associate
/// base url for the IEX server.
#[derive(Clone)]
pub struct Client<'a> {
    /// The underlying Reqwest client used for requests.
    inner: Arc<ReqwestClient>,
    /// The url to which the request are sent.
    url: Cow<'a, str>,
    /// The api token.
    token: Cow<'a, str>,
}

fn env(variable: &str) -> Result<String> {
    env::var(variable).map_err(|e| Error::MissingEnv {
        source: e,
        variable: variable.into(),
    })
}

impl<'a> Client<'a> {
    /// Create a new `Client`.
    pub fn new(url: &'a str, token: &'a str) -> Self {
        let inner = Arc::new(ReqwestClient::new());

        Self {
            inner,
            url: Cow::Borrowed(url),
            token: Cow::Borrowed(token),
        }
    }

    /// Creates a `Client` from environment variables.
    ///
    /// The three environment variables used to instantiate the struct are:
    /// - `IEX_BASE_URL`
    /// - `IEX_TOKEN`
    pub fn from_env() -> Result<Self> {
        let inner = Arc::new(ReqwestClient::new());

        let url = env("IEX_BASE_URL")?;
        let token = env("IEX_TOKEN")?;
        Ok(Self {
            inner,
            url: Cow::Owned(url),
            token: Cow::Owned(token),
        })
    }

    /// Send a `Request` to IEX
    pub async fn send<R: Request>(&self, request: R) -> Result<R::Response> {
        let endpoint = request.endpoint();
        let endpoint = endpoint.trim_matches('/');
        let url = format!("{}/{}", self.url, endpoint);

        let res = self
            .inner
            .request(R::METHOD, &url)
            .headers(request.headers())
            .iex_body(request.body())
            .query(&[("token", &self.token)])
            .send()
            .await?;
        let status = res.status();
        if status.is_success() {
            res.json().map_err(From::from).await
        } else if status.is_client_error() {
            Err(Error::ClientError(status, res.text().await?))
        } else {
            Err(Error::ServerError(status, res.text().await?))
        }
    }

    pub async fn send_all<T, R>(&self, requests: T) -> Vec<Result<R::Response>>
    where
        T: Iterator<Item = R>,
        R: Request,
    {
        stream::iter(requests)
            .map(|r| self.send(r).map_into())
            .filter_map(|x| x)
            .collect()
            .await
    }
}
