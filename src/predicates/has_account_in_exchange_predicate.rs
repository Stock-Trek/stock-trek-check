use crate::{
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use digdigdig3::ExchangeId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HasAccountInExchangePredicate {
    exchange: ExchangeId,
}

impl HasAccountInExchangePredicate {
    pub fn new(exchange: ExchangeId) -> Predicate {
        Box::new(Self { exchange })
    }
}

#[typetag::serde]
impl PredicateTrait for HasAccountInExchangePredicate {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.has_account_in_exchange(&self.exchange))
    }
}
