use std::fs::File;
use std::io::{self, BufRead};
use once_cell::sync::Lazy;
use atty::Stream;

pub fn get_lines(
    args: &Vec<String>,
) -> Result<either::Either<io::Lines<io::BufReader<File>>, io::Lines<io::StdinLock>>, io::Error> {
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
