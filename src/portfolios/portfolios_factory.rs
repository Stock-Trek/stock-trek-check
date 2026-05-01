use crate::portfolios::{in_memory_portfolio::InMemoryPortfolio, portfolio::Portfolio};

pub struct PortfoliosFactory {}

impl PortfoliosFactory {
    pub fn in_memory() -> Portfolio {
        InMemoryPortfolio::new()
    }
}
