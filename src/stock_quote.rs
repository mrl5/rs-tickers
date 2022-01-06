use serde::{Serialize, Deserialize};

mod service;
mod service_stooq;
mod service_yahoo;

#[derive(Serialize, Deserialize, Debug)]
pub struct StockQuote {
    symbol: String,
    source: service::QuoteService,
    ticker: Option<String>,
}

impl StockQuote {
    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }

    pub fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let service = service::get_service(&self.source);
        service.fetch_price(client, &self.id())
    }

    fn id(&self) -> &String {
        match &self.ticker {
            Some(t) => t,
            None => &self.symbol,
        }
    }
}
