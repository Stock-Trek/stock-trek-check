use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetTotalValue {
    asset_id_value: AssetIdValue,
}

impl AssetTotalValue {
    pub fn new(asset_id_value: AssetIdValue) -> NumberValue {
        Box::new(Self { asset_id_value })
    }
}

#[typetag::serde]
impl NumberValueTrait for AssetTotalValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let asset_id = self.asset_id_value.asset_id(c)?;
        Ok(c.portfolio.asset_total(&asset_id))
    }
}
