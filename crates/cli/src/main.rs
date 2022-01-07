use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use structopt::{clap::AppSettings, StructOpt};
use rs_tickers::http;
use rs_tickers::stock_quote;

mod input;

const OUT_DIR: &str = "/tmp/watchlist_quotes";

#[derive(StructOpt)]
#[structopt(
    name = "rs-tickers",
    about = "Gets stock quotes for provided tickers",
    global_settings(&[
      AppSettings::ColoredHelp
    ]),
)]
pub struct CliOptions {
    #[structopt(parse(from_os_str))]
    pub(crate) json_path: Option<PathBuf>,
}

fn main() {
    let options = CliOptions::from_args();

    process::exit(match run_app(options) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            1
        }
    });
}

fn run_app(opts: CliOptions) -> Result<(), io::Error> {
    fs::create_dir_all(OUT_DIR)?;
    let client = http::get_client().unwrap();

    for line in input::get_lines(opts)? {
        let sq: stock_quote::StockQuote = match serde_json::from_str(&line?) {
            Ok(json) => json,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        };

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
