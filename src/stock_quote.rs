use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StockQuote {
    symbol: String,
    source: QuoteSource,
    ticker: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")] // https://stackoverflow.com/a/59167858
pub enum QuoteSource {
    Yahoo,
}

impl StockQuote {
    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }

    pub fn get_ticker(&self) -> &Option<String> {
        &self.ticker
    }
}
