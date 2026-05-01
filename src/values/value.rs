use crate::resolved_context::ResolvedContext;
use anyhow::Result;
use digdigdig3::{Asset, ExchangeId};

pub type AssetValue = Box<dyn AssetValueTrait>;
pub type ExchangeValue = Box<dyn ExchangeValueTrait>;
pub type FlagValue = Box<dyn FlagValueTrait>;
pub type NumberValue = Box<dyn NumberValueTrait>;

#[typetag::serde]
pub trait AssetValueTrait: Send + Sync {
    fn asset(&self, context: &ResolvedContext) -> Result<Asset>;
}

#[typetag::serde]
pub trait ExchangeValueTrait: Send + Sync {
    fn exchange(&self, context: &ResolvedContext) -> Result<ExchangeId>;
}

#[typetag::serde]
pub trait FlagValueTrait: Send + Sync {
    fn flag(&self, context: &ResolvedContext) -> Result<bool>;
}

#[typetag::serde]
pub trait NumberValueTrait: Send + Sync {
    fn number(&self, context: &ResolvedContext) -> Result<f64>;
}
