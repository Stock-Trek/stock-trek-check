use crate::error::{
    general::GeneralError, portfolio::PortfolioError, stats::StatsError, value::ValueError,
    verification::VerificationError,
};
use std::fmt;

pub type StockTrekResult<T> = Result<T, StockTrekError>;

#[derive(Debug)]
pub enum StockTrekError {
    General(GeneralError),
    Portfolio(PortfolioError),
    Stats(StatsError),
    Value(ValueError),
    Verification(VerificationError),
}

impl fmt::Display for StockTrekError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StockTrekError::General(e) => write!(f, "General error: {}", e),
            StockTrekError::Portfolio(e) => write!(f, "Portfolio error: {}", e),
            StockTrekError::Stats(e) => write!(f, "Stats error: {}", e),
            StockTrekError::Value(e) => write!(f, "Value error: {}", e),
            StockTrekError::Verification(e) => write!(f, "Verification error: {}", e),
        }
    }
}

impl std::error::Error for StockTrekError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StockTrekError::General(e) => Some(e),
            StockTrekError::Portfolio(e) => Some(e),
            StockTrekError::Stats(e) => Some(e),
            StockTrekError::Value(e) => Some(e),
            StockTrekError::Verification(e) => Some(e),
        }
    }
}
