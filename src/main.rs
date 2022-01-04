use std::env;
use std::io;

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
    for line in input::get_lines(args)? {
        let deserialized_json: stock_quote::StockQuote = serde_json::from_str(&line?).unwrap();
        println!("{:?}", deserialized_json);
    }
    Ok(())
}

mod input;
mod stock_quote;
