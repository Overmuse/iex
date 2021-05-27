use serde::Serialize;

pub mod client;
pub mod dividends;
pub mod errors;
pub mod request;
pub mod splits;

#[derive(Debug, Clone, Serialize)]
pub enum Range {
    #[serde(rename = "5y")]
    FiveYears,
    #[serde(rename = "2y")]
    TwoYears,
    #[serde(rename = "1y")]
    OneYear,
    #[serde(rename = "ytd")]
    YearToDate,
    #[serde(rename = "6m")]
    SixMonths,
    #[serde(rename = "3m")]
    ThreeMonths,
    #[serde(rename = "1m")]
    OneMonth,
    #[serde(rename = "next")]
    Next,
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Range::FiveYears => write!(f, "5y"),
            Range::TwoYears => write!(f, "2y"),
            Range::OneYear => write!(f, "1y"),
            Range::YearToDate => write!(f, "ytd"),
            Range::SixMonths => write!(f, "6m"),
            Range::ThreeMonths => write!(f, "3m"),
            Range::OneMonth => write!(f, "1m"),
            Range::Next => write!(f, "next"),
        }
    }
}
