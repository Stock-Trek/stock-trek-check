# stock-trek

A lightweight, composable time series and statistical toolkit designed for running crypto bots on [stock-trek.com](https://stock-trek.com). Rust-native core.

## Overview

stock-trek provides core abstractions and utilities for working with market data, building trading strategies, and performing statistical analysis. Key capabilities include:

**Market Data**
- Order books (bids/asks per exchange)
- Ticks (last-traded price/quantity with bid/ask)
- Quotes (price-quantity pairs)
- OHLCV candles (open, high, low, close, volume)
- Aligned and rolling time windows
- Timestamps (millisecond precision)

**Statistics & Analysis**
- **Descriptive statistics**: mean, variance, standard deviation, covariance, correlation, skewness, kurtosis
- **Time series analysis**: autocorrelation, autocovariance, cross-correlation, partial autocorrelation
- **Moving averages**: simple (SMA), exponential (EMA), weighted (WMA)
- **Exponential smoothing**: simple, Holt linear trend, Holt-Winters (additive/multiplicative)
- **Decomposition**: classical seasonal decompose, STL (LOESS-based), LOESS smoothing
- **Filters**: Hodrick-Prescott, Wiener filter
- **Frequency domain**: discrete Fourier transform (DFT), inverse DFT, periodogram, spectral density
- **Wavelets**: continuous wavelet transform (Morlet, Mexican hat), discrete wavelet transform (Haar)
- **Hypothesis testing**: augmented Dickey-Fuller, Jarque-Bera, KPSS, Durbin-Watson, Ljung-Box
- **Model evaluation**: AIC, BIC, log-likelihood, MAE, MAPE, MSE, RMSE
- **Transformations**: Box-Cox, detrend (linear), difference, lag, logarithm, rolling mean, rolling standard deviation, seasonal difference
- **Advanced statistics**: Hurst exponent, mutual information, sample entropy, Shannon entropy

**Backtesting & Strategy Framework**
- `Strategy` trait with `#[register_strategy(default)]` annotation
- `ScratchKey` / `ScratchPad` system for inter-call state
- `ResolverContext` / `ResolvedContext` for order resolution
- Rich predicate system: compare, scratch pad, owns asset, has account, quantity of, and not
- Full order model: single, OCO, OTO, OTOCO orders with limit/market pricing, immediate/triggered activation, fill policies, and time-in-force constraints
- Exchanges, portfolios, and asset value tracking

**Code Verification**
- Syntax verifier for strategy code (disallows unsupported constructs)
- CLI tool (`stock-trek verify --file <path>`)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
stock-trek = "0.6.7"
```

## Python Bindings

Python bindings are available for installation via pip:

```sh
pip install stock-trek
```

## Usage

### Implementing a Strategy

Implement the `Strategy` and `Default` traits for your algorithm and register it with the annotation `#[register_strategy(default)]`.
An example implementing a cost averaging strategy follows:

```rs
use stock_trek::prelude::*;
use std::cmp::Ordering;

pub struct CostAveraging {
    key_exchange: ScratchKey<ExchangeId>,
    key_market_exists: ScratchKey<bool>,
    key_satoshi_price: ScratchKey<f64>,
    key_satoshi_quantity: ScratchKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            key_exchange: ScratchKey::new_required("EXCHANGE"),
            key_market_exists: ScratchKey::new_optional("MARKET_EXISTS", false),
            key_satoshi_price: ScratchKey::new_required("SATOSHI_PRICE"),
            key_satoshi_quantity: ScratchKey::new_required("SATOSHI_QUANTITY"),
        }
    }
}

#[register_strategy(default)]
impl Strategy for CostAveraging {
    fn preferences(&self) -> Preferences {
        Preferences {
            multi_leg: MultiLeg {
                if_different_price_unsupported: OnDifferent::UseDataFromPrimary,
                if_different_symbol_unsupported: OnDifferent::UseDataFromPrimary,
            },
        }
    }
    fn calculate(&self, c: &StrategyContext) -> ScratchPad {
        let mut scratch_pad = ScratchPad::new();
        let one_millionth = 1.0 / 1_000_000.0;
        scratch_pad.write(&self.key_satoshi_quantity, one_millionth);
        let iter = c.exchange_markets_for(&AssetId::Bitcoin, &AssetId::Bitcoin);
        let min_by_last_ask = iter.min_by(|(_a_exch, a_market), (_b_exch, b_market)| {
            let a_last_ask = a_market.ticks.ticks[0].ask.price;
            let b_last_ask = b_market.ticks.ticks[0].ask.price;
            a_last_ask.partial_cmp(&b_last_ask).unwrap()
        });
        if let Some((cheapest_exchange_name, market)) = min_by_last_ask {
            scratch_pad.write(&self.key_exchange, cheapest_exchange_name);
            scratch_pad.write(&self.key_market_exists, true);
            let satoshi_price = market.ticks.ticks[0].ask.price / 1_000_000.0;
            scratch_pad.write(&self.key_satoshi_price, satoshi_price);
        }
        scratch_pad
    }
    fn resolver(&self, c: &ResolverContext) -> Resolver {
        let exchange = c.scratch_pad.exchange_id(&self.key_exchange);
        let btc = c.literals.asset_id(AssetId::Bitcoin);
        let usdt = c.literals.asset_id(AssetId::Tether);
        let satoshi_price = c.scratch_pad.number(&self.key_satoshi_price);
        let quantity = c.scratch_pad.number(&self.key_satoshi_quantity);
        c.resolvers.if_else(
            c.predicates.scratch_pad(&self.key_market_exists),
            c.resolvers.if_else(
                c.predicates.compare(
                    c.portfolio
                        .asset_in_exchange(exchange.clone(), usdt.clone()),
                    Ordering::Greater,
                    satoshi_price,
                ),
                c.resolvers.enqueue_order(
                    exchange.clone(),
                    c.orders.single(
                        btc,
                        usdt.clone(),
                        OrderIntent::Open,
                        OrderSide::Buy,
                        OrderActivation::Immediate,
                        OrderPricing::Market,
                        OrderQuantity::OfQuote(quantity),
                        vec![OrderConstraint::FillPolicy {
                            allow_partial: true,
                        }],
                    ),
                ),
                c.resolvers.no_op(),
            ),
            c.resolvers.no_op(),
        )
    }
}
```

### Statistics

```rs
use stock_trek::statistics::{Stats, moving_average::MovingAverage};

let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let stats = Stats::default();

let mean = stats.mean(&values)?;                   // 3.0
let variance = stats.variance(&values, 0)?;         // 2.0
let std_dev = stats.standard_deviation(&values, 0)?; // ~1.414

let ma = MovingAverage;
let sma = ma.simple_moving_average(&values, 3);     // [2.0, 3.0, 4.0]
let ema = ma.exponential_moving_average(&values, 0.5); // [1.0, 1.5, 2.25, ...]
```

### Code Verification

Stock-Trek verifies code before running it and disallows certain syntax elements. To verify code locally, install it with

```sh
cargo install stock-trek
```

then run the verify command with

```sh
stock-trek verify --file ./path/strategy.rs
```

## Modules

| Module | Description |
|--------|-------------|
| `market_data` | Market data types: ticks, quotes, OHLCV, order books, aligned/rolling windows, timestamps |
| `statistics` | Statistical and time series analysis (see breakdown below) |
| `strategy` | `Strategy` trait and `#[register_strategy]` annotation |
| `strategy_context` | Context provided to strategies at calculation time |
| `resolver_context` / `resolved_context` | Contexts for order resolution |
| `resolvers` | Order resolution primitives: if-else, enqueue order, no-op, list |
| `predicates` | Conditional predicates: compare, scratch pad, owns asset, has account, etc. |
| `order` | Full order model: single, OCO, OTO, OTOCO, pricing, activation, constraints |
| `portfolios` | Portfolio tracking (in-memory, stub) |
| `scratch` | Scratch key/value system for inter-call state |
| `values` | Value system: literals, scratch pad values, calculations, asset values |
| `exchange_id` | Exchange identifier type |
| `asset_id` | Asset identifier type (Bitcoin, Tether, etc.) |
| `preferences` | Strategy preferences (multi-leg behavior) |
| `capability` | Exchange capability definitions |
| `verification` | Syntax verification and policy enforcement |
| `examples` | Example strategies (cost averaging) |

### Statistics Module Breakdown

| Sub-module | Features |
|------------|----------|
| `stats` | Mean, variance, standard deviation, covariance, correlation, skewness, kurtosis |
| `moving_average` | SMA, EMA, weighted moving average |
| `time_series` | Autocorrelation, autocovariance, cross-correlation, partial autocorrelation |
| `exponential_smoothing` | Simple exponential smoothing, Holt linear trend, Holt-Winters |
| `decompose` | LOESS smoothing, seasonal decompose, STL decomposition |
| `filter` | Hodrick-Prescott filter, Wiener filter |
| `frequency` | DFT, inverse DFT, periodogram, spectral density |
| `wavelet` | Continuous wavelet transform (Morlet, Mexican hat), discrete wavelet transform (Haar) |
| `hypothesis` | ADF test, Jarque-Bera test, KPSS test, Durbin-Watson, Ljung-Box |
| `evaluation` | AIC, BIC, log-likelihood, MAE, MAPE, MSE, RMSE |
| `transformation` | Box-Cox, detrend, difference, lag, logarithm, rolling mean/std, seasonal difference |
| `advanced` | Hurst exponent, mutual information, sample entropy, Shannon entropy |

## Roadmap

Planned features include:

- Technical indicators (EMA, RSI, MACD, etc.)
- Enhanced backtesting and simulation utilities

## Status

This project is in early development (0.x). APIs may change.

## License

MIT
