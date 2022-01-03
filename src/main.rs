use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    if args.len() > 2 {
        let filename = &args[2];
        read_from_file(filename)
    } else {
        Ok(())
    }
}

fn read_from_file<P>(filename: P) -> Result<(), io::Error>
where P: AsRef<Path>, {
    for line in read_lines(filename)? {
        if let Ok(l) = line {
            println!("{}", l);
        }
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
