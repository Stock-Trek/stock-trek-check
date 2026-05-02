use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum PortfolioError {
    #[error("Portfolio has no account in {exchange}")]
    NoAccountInExchange { exchange: String },
    #[error("Portfolio does not own any {asset} in {exchange}")]
    AssetNotOwned { exchange: String, asset: String },
    #[error("Portfolio has {tokens} {asset} in {exchange} and cannot remove {remove_request}")]
    NotEnoughTokens {
        exchange: String,
        asset: String,
        tokens: f64,
        remove_request: f64,
    },
}
