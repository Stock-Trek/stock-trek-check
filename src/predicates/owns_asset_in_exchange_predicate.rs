use crate::{
    asset_id::AssetId,
    error::result::StockTrekResult,
    exchange_id::ExchangeId,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetInExchangePredicate {
    exchange_id: ExchangeId,
    asset_id: AssetId,
}

impl OwnsAssetInExchangePredicate {
    pub fn new(asset_id: AssetId, exchange_id: ExchangeId) -> Predicate {
        Box::new(Self {
            asset_id,
            exchange_id,
        })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsAssetInExchangePredicate {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio
            .owns_asset_in_exchange(&self.asset_id, &self.exchange_id))
    }
}
