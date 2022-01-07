use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use atty::Stream;
use once_cell::sync::Lazy;

static STDIN: Lazy<io::Stdin> = Lazy::new(io::stdin);

pub fn get_lines(
    path: Option<PathBuf>,
) -> Result<
    either::Either<io::Lines<io::BufReader<File>>, io::Lines<io::StdinLock<'static>>>,
    io::Error,
> {
    let lines;

    match path {
        Some(p) => {
            let file = File::open(p)?;
            let l = io::BufReader::new(file).lines();
            lines = either::Either::Left(l);
        }
        None => {
            if atty::is(Stream::Stdin) {
                return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
            }

            let l = STDIN.lock().lines();
            lines = either::Either::Right(l);
        }
    }

    Ok(lines)
}
