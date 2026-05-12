use crate::{
    error::result::StockTrekResult,
    exchange_id::ExchangeId,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HasAccountInExchangePredicate {
    exchange_id: ExchangeId,
}

impl HasAccountInExchangePredicate {
    pub fn new(exchange_id: ExchangeId) -> Predicate {
        Box::new(Self { exchange_id })
    }
}

#[typetag::serde]
impl PredicateTrait for HasAccountInExchangePredicate {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.has_account_in_exchange(&self.exchange_id))
    }
}
