pub mod http;
pub mod stock_quote;

#[cfg(test)]
mod tests {
    use crate::stock_quote::StockQuote;

    #[test]
    fn it_should_serialize_from_json() {
        let test_input = [
            r#"{"symbol":"aapl","source":"yahoo"}"#,
            r#"{"symbol":"cdr","source":"stooq"}"#,
            r#"{"symbol":"gbpusd","source":"yahoo","ticker":"gbpusd=x"}"#,
        ];
        for json in test_input {
            let sq: Result<StockQuote, serde_json::error::Error> = serde_json::from_str(json);
            assert!(sq.is_ok());
        }
    }
}
