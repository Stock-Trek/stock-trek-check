# stock-trek-check

A local verifier for checking rust code is valid for running crypto bots on [stock-trek.com](https://stock-trek.com)

## Installation

Stock-Trek verifies code before running it and disallows certain syntax elements. To verify code will be valid locally, install stock-trek-check with

```sh
cargo install stock-trek-check
```

then run the verify command with

```sh
stock-trek-check verify --file ./path/algorithm.rs
```

## Status

This project is in early development so APIs may change.

## License

MIT
