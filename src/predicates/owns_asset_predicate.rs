use crate::{
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use digdigdig3::Asset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OwnsAssetPredicate {
    asset: Asset,
}

impl OwnsAssetPredicate {
    pub fn new(asset: Asset) -> Predicate {
        Box::new(Self { asset })
    }
}

#[typetag::serde]
impl PredicateTrait for OwnsAssetPredicate {
    fn test(&self, context: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(context.portfolio.owns_asset(&self.asset))
    }
}
