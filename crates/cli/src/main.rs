use rs_tickers::http;
use rs_tickers::stock_quote;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::{clap::AppSettings, StructOpt};

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
    out_dir: PathBuf,
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

fn run_app(opts: CliOptions) -> Result<(), Box<dyn Error>> {
    let out_dir = opts.out_dir;
    let client = http::get_client()?;

    fs::create_dir_all(&out_dir)?;
    for line in input::get_lines(opts.json_path)? {
        let sq: stock_quote::StockQuote = serde_json::from_str(&line?)?;

        match sq.fetch_price(&client) {
            Ok(p) => {
                if opts.write_to_stdout {
                    output::write_to_stdout(sq.get_symbol(), &p);
                } else {
                    output::write_to_file(out_dir.to_owned(), sq.get_symbol(), &p);
                }
            }
            Err(e) => log::error!("couldn't get price for {}: {}", sq.get_symbol(), e),
        };
    }

    Ok(())
}
