use crate::{
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetInExchangePredicate {
    asset: Asset,
    exchange: ExchangeId,
}

impl OwnsAssetInExchangePredicate {
    pub fn new(asset: Asset, exchange: ExchangeId) -> Predicate {
        Box::new(Self { asset, exchange })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsAssetInExchangePredicate {
    fn test(&self, context: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(context
            .portfolio
            .owns_asset_in_exchange(&self.asset, &self.exchange))
    }
}
