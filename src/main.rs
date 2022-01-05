use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    std::process::exit(match run_app(&args) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            1
        }
    });
}

fn run_app(args: &Vec<String>) -> Result<(), io::Error> {
    let client = http::get_client().unwrap();
    for line in input::get_lines(args)? {
        let sq: stock_quote::StockQuote = serde_json::from_str(&line?).unwrap();
        match sq.get_price(&client) {
            Ok(p) => println!("{}", p),
            Err(e) => eprintln!("couldnt get price for {}: {}", sq.get_symbol(), e),
        };
    }

    Ok(())
}

mod http;
mod input;
mod stock_quote;
