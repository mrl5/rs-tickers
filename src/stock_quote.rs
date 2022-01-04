use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StockQuote {
    symbol: String,
    source: QuoteSource,
    ticker: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")] // https://stackoverflow.com/a/59167858
pub enum QuoteSource {
    Yahoo,
}
