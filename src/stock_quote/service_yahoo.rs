use super::service;

pub struct Yahoo {}

impl Yahoo {
    pub fn new() -> Self {
        Self {}
    }
}

impl service::Fetches for Yahoo {
    fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
        ticker: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            ticker
        );
        let result = client.get(url).send()?.text()?;
        let x: serde_json::Value = serde_json::from_str(&result).unwrap();
        Ok(
            x["chart"]["result"][0]["meta"]["regularMarketPrice"].to_owned(),
        )
    }
}
