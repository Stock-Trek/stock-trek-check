use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    scratch::key::ScratchKey,
    values::value::{AssetValueTrait, ExchangeValueTrait, FlagValueTrait, NumberValueTrait},
};
use digdigdig3::{Asset, ExchangeId};

#[typetag::serde]
impl AssetValueTrait for ScratchKey<Asset> {
    fn asset(&self, c: &ResolvedContext) -> StockTrekResult<Asset> {
        self.read(c)
    }
}
#[typetag::serde]
impl ExchangeValueTrait for ScratchKey<ExchangeId> {
    fn exchange(&self, c: &ResolvedContext) -> StockTrekResult<ExchangeId> {
        self.read(c)
    }
}
#[typetag::serde]
impl FlagValueTrait for ScratchKey<bool> {
    fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        self.read(c)
    }
}
#[typetag::serde]
impl NumberValueTrait for ScratchKey<f64> {
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        self.read(c)
    }
}
