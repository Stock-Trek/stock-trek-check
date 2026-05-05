use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{AssetValue, NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssetTotalValue {
    asset: AssetValue,
}

impl AssetTotalValue {
    pub fn new(asset: AssetValue) -> NumberValue {
        Box::new(Self { asset })
    }
}

#[typetag::serde]
impl NumberValueTrait for AssetTotalValue {
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let asset = self.asset.asset(c)?;
        Ok(c.portfolio.asset_total(&asset))
    }
}
