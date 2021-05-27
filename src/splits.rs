use crate::request::Request;
use crate::Range;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
pub struct GetSplits<'a> {
    pub symbol: &'a str,
    pub range: Range,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Split {
    pub ex_date: NaiveDate,
    pub declared_date: NaiveDate,
    pub ratio: f64,
    pub to_factor: f64,
    pub from_factor: f64,
    pub description: String,
    pub symbol: String,
    pub id: String,
    pub source: Option<String>,
    pub key: String,
    pub subkey: String,
    pub date: Option<usize>,
    pub updated: usize,
}

impl Request for GetSplits<'_> {
    type Body = ();
    type Response = Vec<Split>;

    fn endpoint(&self) -> Cow<str> {
        format!("/stock/{}/splits/{}", self.symbol, self.range).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_splits() {
        let _splits_mock = mock("GET", "/stock/AAPL/splits/next")
            .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("token".into(), "TOKEN".into()),
            ]))
            .with_body(r#"[{"declaredDate":"2017-08-01","description":"7-for-1split","exDate":"2017-08-10","fromFactor":1,"ratio":0.142857,"refid":6910760,"symbol":"AAPL","toFactor":7,"id":"SPLITS","key":"AAPL","subkey":"6846210","updated":1609576419432}]"#)
            .create();
        let url = mockito::server_url();

        let client = Client::new(&url, "TOKEN");
        let req = GetSplits {
            symbol: "AAPL",
            range: Range::Next,
        };
        client.send(req).await.unwrap();
    }
}
