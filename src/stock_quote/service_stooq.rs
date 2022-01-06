use super::service;

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

    fn scrap_price(&self, html: scraper::Html, ticker: &str) -> Option<String> {
        let mut price: Option<String> = None;
        let cryptic_category_ids: Vec<u8> = (2..=5).collect();

        for i in cryptic_category_ids {
            let id = format!("aq_{}_c{}", ticker, i);
            let target = format!(r#"span[id="{}"]"#, id);
            let selector = scraper::Selector::parse(&target).unwrap();

            match html.select(&selector).next() {
                Some(v) => {
                    println!("{}", id);
                    price = Some(v.inner_html());
                    break;
                }
                None => continue,
            }
        }

        price
    }
}

impl service::Fetches for Stooq {
    fn fetch_price(
        &self,
        client: &reqwest::blocking::Client,
        ticker: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("https://stooq.pl/q/?s={}", ticker);
        let result = client.get(url).headers(self.get_headers()).send()?.text()?;
        let html = scraper::Html::parse_fragment(&result);

        Ok(
            serde_json::from_str(&self.scrap_price(html, ticker).unwrap()).unwrap(),
        )
    }
}
