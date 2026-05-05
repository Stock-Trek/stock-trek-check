use crate::{error::result::StockTrekResult, resolved_context::ResolvedContext};
use digdigdig3::{Asset, ExchangeId};

pub type AssetValue = Box<dyn AssetValueTrait>;
pub type ExchangeValue = Box<dyn ExchangeValueTrait>;
pub type FlagValue = Box<dyn FlagValueTrait>;
pub type NumberValue = Box<dyn NumberValueTrait>;

#[typetag::serde]
pub trait AssetValueTrait: Send + Sync {
    fn asset(&self, c: &ResolvedContext) -> StockTrekResult<Asset>;
}

#[typetag::serde]
pub trait ExchangeValueTrait: Send + Sync {
    fn exchange(&self, c: &ResolvedContext) -> StockTrekResult<ExchangeId>;
}

#[typetag::serde]
pub trait FlagValueTrait: Send + Sync {
    fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool>;
}

#[typetag::serde]
pub trait NumberValueTrait: Send + Sync {
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64>;
}
