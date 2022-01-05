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

    pub fn get_price(
        &self,
        client: &reqwest::blocking::Client,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url =
            format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            &self.id(),
        );
        let result = client.get(url).send()?.text()?;
        let x: serde_json::Value = serde_json::from_str(&result).unwrap();
        Ok(
            x["chart"]["result"][0]["meta"]["regularMarketPrice"].to_owned(),
        )
    }

    fn id(&self) -> &String {
        match &self.ticker {
            Some(t) => t,
            None => &self.symbol,
        }
    }
}
