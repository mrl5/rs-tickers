use std::boxed::Box;
use serde::{Serialize, Deserialize};
use super::service_stooq;
use super::service_yahoo;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")] // https://stackoverflow.com/a/59167858
pub enum QuoteService {
    Stooq,
    Yahoo,
}

pub trait Fetches {
    fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
        ticker: &str,
    ) -> Result<serde_json::Value, reqwest::Error>;
}

pub fn get_service(service_type: &QuoteService) -> Box<dyn Fetches> {
    // factory design pattern
    match service_type {
        QuoteService::Stooq => Box::new(service_stooq::Stooq::new()),
        QuoteService::Yahoo => Box::new(service_yahoo::Yahoo::new()),
    }
}
