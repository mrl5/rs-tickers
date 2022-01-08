use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub fn write_to_file(out_dir: &PathBuf, symbol: &str, price: &serde_json::Value) {
    let path = out_dir.join(format!("{}.txt", symbol));
    let mut file = fs::File::create(&path).unwrap();

    log::info!(
        "writing current price of {} to {} ...",
        symbol,
        &path.into_os_string().into_string().unwrap()
    );
    let s = serde_json::to_string(price).unwrap();
    file.write_all(&s.as_bytes()).unwrap();
}

pub fn write_to_stdout(symbol: &str, price: &serde_json::Value) {
    let json = OutJson {
        symbol: symbol.to_owned(),
        price: price.to_owned(),
    };
    println!("{}", serde_json::to_string(&json).unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
struct OutJson {
    symbol: String,
    price: serde_json::Value,
}
