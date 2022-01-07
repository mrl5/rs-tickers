use serde::{Serialize, Deserialize};

mod service_stooq;
mod service_yahoo;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")] // https://stackoverflow.com/a/59167858
pub enum QuoteService {
    Stooq,
    Yahoo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockQuote {
    symbol: String,
    source: QuoteService,
    ticker: Option<String>,
}

impl StockQuote {
    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let service = get_service(&self.source);
        log::info!("fetching current price of {} ...", &self.symbol);
        service.fetch_price(client, &self.id())
    }

    fn id(&self) -> &str {
        match &self.ticker {
            Some(t) => t,
            None => &self.symbol,
        }
    }
}

fn get_service(service_type: &QuoteService) -> Box<dyn Fetches> {
    // factory design pattern
    match service_type {
        QuoteService::Stooq => Box::new(service_stooq::Stooq::new()),
        QuoteService::Yahoo => Box::new(service_yahoo::Yahoo::new()),
    }
}

trait Fetches {
    fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
        ticker: &str,
    ) -> Result<serde_json::Value, reqwest::Error>;
}
