use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;
use structopt::{clap::AppSettings, StructOpt};
use rs_tickers::http;
use rs_tickers::stock_quote;

mod input;
mod output;

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

    #[structopt(short = "o", long = "to-stdout")]
    write_to_stdout: bool,

    #[structopt(short = "d", long = "output-dir", default_value = DEFAULT_OUT_DIR)]
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
            Ok(p) => if opts.write_to_stdout {
                output::write_to_stdout(sq.get_symbol(), &p);
            } else {
                output::write_to_file(&out_dir, sq.get_symbol(), &p);
            },
            Err(e) => log::error!("couldnt get price for {}: {}", sq.get_symbol(), e),
        };
    }

    Ok(())
}
