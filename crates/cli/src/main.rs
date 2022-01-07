use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use structopt::{clap::AppSettings, StructOpt};
use rs_tickers::http;
use rs_tickers::stock_quote;

mod input;

const DEFAULT_OUT_DIR: &str = "/tmp/watchlist_quotes";

#[derive(StructOpt)]
#[structopt(
    name = "rs-tickers",
    about = "Gets stock quotes for provided tickers",
    global_settings(&[
      AppSettings::ColoredHelp
    ]),
)]
struct CliOptions {
    #[structopt(parse(from_os_str))]
    pub(crate) json_path: Option<PathBuf>,

    #[structopt(short = "o", long = "output-dir", default_value = DEFAULT_OUT_DIR)]
    out_dir: String,
}

fn main() {
    env_logger::init();
    log::debug!("initialized logger");

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
    let out_dir = opts.out_dir.to_owned();
    let client = http::get_client().unwrap();

    fs::create_dir_all(&out_dir)?;
    for line in input::get_lines(opts.json_path)? {
        let sq: stock_quote::StockQuote = match serde_json::from_str(&line?) {
            Ok(json) => json,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        };

        match sq.fetch_price(&client) {
            Ok(p) => write_price(&out_dir, sq.get_symbol(), &p),
            Err(e) => log::error!("couldnt get price for {}: {}", sq.get_symbol(), e),
        };
    }

    Ok(())
}

fn write_price(out_dir: &str, symbol: &str, price: &serde_json::Value) {
    let path = format!("{}/{}.txt", out_dir, symbol);
    let mut file = fs::File::create(&path).unwrap();

    log::info!("writing current price of {} to {} ...", symbol, &path);
    let s = serde_json::to_string(price).unwrap();
    file.write_all(&s.as_bytes()).unwrap();
}
