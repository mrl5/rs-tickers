use std::error::Error;
use std::fmt::Display;

pub struct Stooq {}

impl Stooq {
    pub fn new() -> Self {
        Self {}
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT, "text/html".parse().unwrap());
        headers.insert(
            reqwest::header::COOKIE,
            "FCCDCF=[]; FCNEC=[]".parse().unwrap(),
        );

        headers
    }

    fn scrap_price(&self, html: scraper::Html, ticker: &str) -> Result<String, SelectorError> {
        let mut price: Option<String> = None;
        let cryptic_category_ids: Vec<u8> = (2..=5).collect();

        for i in cryptic_category_ids {
            let id = format!("aq_{}_c{}", ticker, i);
            let target = format!(r#"span[id="{}"]"#, id);
            let selector = scraper::Selector::parse(&target).unwrap();

            match html.select(&selector).next() {
                Some(v) => {
                    price = Some(v.inner_html());
                    break;
                }
                None => continue,
            }
        }

        match price {
            Some(v) => Ok(v),
            None => Err(SelectorError::new(&format!(
                "couldn't scrap price for {}",
                ticker
            ))),
        }
    }
}

impl super::Fetches for Stooq {
    fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
        ticker: &str,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("https://stooq.pl/q/?s={}", ticker);
        let result = client.get(url).headers(self.get_headers()).send()?.text()?;
        let html = scraper::Html::parse_fragment(&result);

        let price = self.scrap_price(html, ticker)?;
        Ok(serde_json::from_str(&price)?)
    }
}

#[derive(Debug)]
struct SelectorError {
    msg: String,
}

impl SelectorError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl Error for SelectorError {}

impl Display for SelectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}
