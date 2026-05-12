use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetInExchangeValue {
    exchange_id_value: ExchangeIdValue,
    asset_id_value: AssetIdValue,
}

impl AssetInExchangeValue {
    pub fn new(exchange_id_value: ExchangeIdValue, asset_id_value: AssetIdValue) -> NumberValue {
        Box::new(Self {
            exchange_id_value,
            asset_id_value,
        })
    }
}

#[typetag::serde]
impl NumberValueTrait for AssetInExchangeValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let exchange_id = self.exchange_id_value.exchange_id(c)?;
        let asset_id = self.asset_id_value.asset_id(c)?;
        Ok(c.portfolio.asset_in_exchange(&asset_id, &exchange_id))
    }
}
