use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{
        AssetValue, AssetValueTrait, ExchangeValue, ExchangeValueTrait, FlagValue, FlagValueTrait,
        NumberValue, NumberValueTrait,
    },
};
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LiteralAssetValue {
    literal: Asset,
}
#[derive(Serialize, Deserialize)]
pub struct LiteralExchangeValue {
    literal: ExchangeId,
}
#[derive(Serialize, Deserialize)]
pub struct LiteralFlagValue {
    literal: bool,
}
#[derive(Serialize, Deserialize)]
pub struct LiteralNumberValue {
    literal: f64,
}

impl LiteralAssetValue {
    pub fn new(literal: Asset) -> AssetValue {
        Box::new(Self { literal })
    }
}
impl LiteralExchangeValue {
    pub fn new(literal: ExchangeId) -> ExchangeValue {
        Box::new(Self { literal })
    }
}
impl LiteralFlagValue {
    pub fn new(literal: bool) -> FlagValue {
        Box::new(Self { literal })
    }
}
impl LiteralNumberValue {
    pub fn new(literal: f64) -> NumberValue {
        Box::new(Self { literal })
    }
}

#[typetag::serde]
impl AssetValueTrait for LiteralAssetValue {
    fn asset(&self, _: &ResolvedContext) -> StockTrekResult<Asset> {
        Ok(self.literal.clone())
    }
}
#[typetag::serde]
impl ExchangeValueTrait for LiteralExchangeValue {
    fn exchange(&self, _: &ResolvedContext) -> StockTrekResult<ExchangeId> {
        Ok(self.literal)
    }
}
#[typetag::serde]
impl FlagValueTrait for LiteralFlagValue {
    fn flag(&self, _: &ResolvedContext) -> StockTrekResult<bool> {
        Ok(self.literal)
    }
}
#[typetag::serde]
impl NumberValueTrait for LiteralNumberValue {
    fn number(&self, _: &ResolvedContext) -> StockTrekResult<f64> {
        Ok(self.literal)
    }
}
