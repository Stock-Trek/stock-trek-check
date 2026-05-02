use crate::error::{
    general::GeneralError, portfolio::PortfolioError, stats::StatsError, value::ValueError,
    verification::VerificationError,
};

pub type StockTrekResult<T> = Result<T, StockTrekError>;

#[derive(Debug)]
pub enum StockTrekError {
    General(GeneralError),
    Portfolio(PortfolioError),
    Stats(StatsError),
    Value(ValueError),
    Verification(VerificationError),
}
