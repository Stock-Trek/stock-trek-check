use crate::{
    asset_id::AssetId,
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetPredicate {
    asset_id: AssetId,
}

impl OwnsAssetPredicate {
    pub fn new(asset_id: AssetId) -> Predicate {
        Box::new(Self { asset_id })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsAssetPredicate {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(c.portfolio.owns_asset(&self.asset_id))
    }
}
