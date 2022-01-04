use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use once_cell::sync::Lazy;
use atty::Stream;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct StockQuote {
    symbol: String,
    source: QuoteSource,
    ticker: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")] // https://stackoverflow.com/a/59167858 -> https://serde.rs/container-attrs.html#rename_all
enum QuoteSource {
    Yahoo
}

fn main() {
    let args: Vec<String> = env::args().collect();
    std::process::exit(match run_app(&args) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}

fn run_app(args: &Vec<String>) -> Result<(), io::Error> {
    for line in get_lines(args)? {
        let deserialized_json: StockQuote = serde_json::from_str(&line?).unwrap();
        println!("{:?}", deserialized_json);
    }
    Ok(())
}

fn get_lines(args: &Vec<String>) -> Result<either::Either<io::Lines<io::BufReader<File>>, io::Lines<io::StdinLock>>, io::Error> {
    if args.len() > 2 {
        let file = File::open(&args[2])?;
        Ok(either::Either::Left(io::BufReader::new(file).lines()))
    } else {
        if atty::is(Stream::Stdin) {
            return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
        }
        static STDIN: Lazy<io::Stdin> = Lazy::new(io::stdin);
        Ok(either::Either::Right(STDIN.lock().lines()))
    }
}
