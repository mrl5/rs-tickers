use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;

mod http;
mod input;
mod stock_quote;

const OUT_DIR: &str = "/tmp/watchlist_quotes";

fn main() {
    let args: Vec<String> = env::args().collect();

    process::exit(match run_app(&args) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            1
        }
    });
}

fn run_app(args: &Vec<String>) -> Result<(), io::Error> {
    fs::create_dir_all(OUT_DIR)?;
    let client = http::get_client().unwrap();

    for line in input::get_lines(args)? {
        let sq: stock_quote::StockQuote = serde_json::from_str(&line?).unwrap();

        println!("[i] fetching current price of {} ...", sq.get_symbol());
        match sq.fetch_price(&client) {
            Ok(p) => write_price(sq.get_symbol(), &p),
            Err(e) => eprintln!("couldnt get price for {}: {}", sq.get_symbol(), e),
        };
    }

    Ok(())
}

fn write_price(symbol: &str, price: &serde_json::Value) {
    let path = format!("{}/{}.txt", OUT_DIR, symbol);
    let mut file = fs::File::create(&path).unwrap();

    println!("[i] writing current price of {} to {} ...", symbol, &path);
    let s = serde_json::to_string(price).unwrap();
    file.write_all(&s.as_bytes()).unwrap();
}
