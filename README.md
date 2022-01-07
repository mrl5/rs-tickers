# rs-tickers

Gets stock quotes for provided tickers


## Use case(s)

1. As an investor I want to provide recent prices to my investment tracking
   spreadsheet


## Usage

```bash
$ rs-tickers --help
```

To see logs use `RUST_LOG` env variable. Possible values are defined in
https://docs.rs/log/latest/log/enum.Level.html

### Fetching stock quotes from CLI

You can pass path to a file ...
```bash
$ RUST_LOG=info rs-tickers example/watchlist.txt
```

... or pipe data from another process
```bash
$ cat example/watchlist.json | jq -c '.[]' | rs-tickers
```

It will create `ticker.txt` files in `/tmp/watchlist_quotes` (unless other dir
is given) that later can be read by the spreadsheet.


### UC1: Integration with your favorite spreadsheet program

After fetching stock quotes their values can be loaded from file into a
spreadsheet cell like this:
```
='file:///tmp/watchlist_quotes/ticker.txt'#$ticker.A1
```


## HOWTO build

```bash
$ cargo build --release
```

Your binary will be in `target/release/cli`.
