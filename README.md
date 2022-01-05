# rs-tickers

Gets stock quotes for provided tickers


## Usage

You can pass path to `tickers.json`
```bash
rs-tickers path/to/tickers.json
```

... or pipe it from another process
```bash
cat path/to/tickers-as-an-json-array.json | jq -c '.[]' | rs-tickers
```
