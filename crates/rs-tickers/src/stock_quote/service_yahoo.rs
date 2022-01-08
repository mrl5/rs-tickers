use std::error::Error;

pub struct Yahoo {}

impl Yahoo {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Fetches for Yahoo {
    fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
        ticker: &str,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            ticker,
        );
        let result = client.get(url).send()?.text()?;
        let x: serde_json::Value = serde_json::from_str(&result)?;

        let price = &x["chart"]["result"][0]["meta"]["regularMarketPrice"];
        Ok(price.to_owned())
    }
}
