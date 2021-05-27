use crate::request::Request;
use crate::Range;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
pub struct GetDividends<'a> {
    pub symbol: &'a str,
    pub range: Range,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Flag {
    Autocall,
    #[serde(rename = "Cash&Stock")]
    CashAndStock,
    Cash,
    DissenterRights,
    Interest,
    Maturity,
    Rebate,
    Stock,
    Special,
    ToBeAnnounced,
    Blank,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dividend {
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub declared_date: NaiveDate,
    pub ex_date: NaiveDate,
    pub flag: Flag,
    pub frequency: String,
    pub payment_date: NaiveDate,
    pub record_date: NaiveDate,
    pub refid: usize,
    pub symbol: String,
    pub id: String,
    pub key: String,
    pub subkey: String,
    pub date: usize,
    pub updated: usize,
}

impl Request for GetDividends<'_> {
    type Body = ();
    type Response = Vec<Dividend>;

    fn endpoint(&self) -> Cow<str> {
        format!("/stock/{}/dividends/{}", self.symbol, self.range).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_dividends() {
        let _dividends_mock = mock("GET", "/stock/AAPL/dividends/next")
            .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("token".into(), "TOKEN".into()),
            ]))
            .with_body(r#"[{"amount":0.70919585493507512,"currency":"USD","declaredDate":"2020-10-19","description":"OrdinaryShares","exDate":"2020-10-28","flag":"Cash","frequency":"quarterly","paymentDate":"2020-11-06","recordDate":"2020-10-28","refid":2096218,"symbol":"AAPL","id":"DIVIDENDS","key":"AAPL","subkey":"2053393","date":1612392166191,"updated":1612392166191}]"#)
            .create();
        let url = mockito::server_url();

        let client = Client::new(&url, "TOKEN");
        let req = GetDividends {
            symbol: "AAPL",
            range: Range::Next,
        };
        client.send(req).await.unwrap();
    }
}
