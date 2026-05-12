use crate::{
    asset_id::AssetId,
    error::result::StockTrekResult,
    exchange_id::ExchangeId,
    resolved_context::ResolvedContext,
    scratch::key::ScratchKey,
    values::value::{
        AssetIdValue, AssetIdValueTrait, ExchangeIdValue, ExchangeIdValueTrait, FlagValue,
        FlagValueTrait, NumberValue, NumberValueTrait,
    },
};

macro_rules! scratch_pad_value {
    ($trait_name:ident, $raw_type:ident, $getter:ident, $clone_type:ident) => {
        #[typetag::serde]
        impl $trait_name for ScratchKey<$raw_type> {
            fn clone_box(&self) -> $clone_type {
                Box::new(self.clone())
            }
            fn $getter(&self, c: &ResolvedContext) -> StockTrekResult<$raw_type> {
                self.read(c)
            }
        }
    };
}

scratch_pad_value! {ExchangeIdValueTrait, ExchangeId, exchange_id, ExchangeIdValue}
scratch_pad_value! {AssetIdValueTrait, AssetId, asset_id, AssetIdValue}
scratch_pad_value! {FlagValueTrait, bool, flag, FlagValue}
scratch_pad_value! {NumberValueTrait, f64, number, NumberValue}
