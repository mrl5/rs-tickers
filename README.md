# rs-tickers

Gets stock quotes for provided tickers


## Use case(s)

1. As an investor I want to provide recent prices to my investment tracking
   spreadsheet


## Usage

You can pass path to a file ...
```bash
$ rs-tickers example/watchlist.txt
```

... or pipe data from another process
```bash
$ cat example/watchlist.json | jq -c '.[]' | rs-tickers
```

It will create `ticker.txt` files in `/tmp/watchlist_quotes` that later can be
read by the spreadsheet.


## HOWTO build

```bash
$ cargo build
```

Your binary will be in `target/debug/rs-tickers`.
