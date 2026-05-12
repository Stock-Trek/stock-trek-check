use crate::{
    asset_id::AssetId,
    error::result::StockTrekResult,
    execute::{
        capability::{Capability, HasRequiredCapabilities},
        increment_sizes::IncrementSizes,
    },
    order::{
        orders::{
            one_cancels_other::OneCancelsOtherOrderGeneric,
            one_triggers_oco::OneTriggersOcoOrderGeneric,
            one_triggers_other::OneTriggersOtherOrderGeneric, single::SingleOrderGeneric,
        },
        trading_pair::TradingPair,
    },
    resolved_context::ResolvedContext,
    resolvers::resolveable::Resolvable,
    values::value::{AssetIdValue, NumberValue},
};
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum OrderRequest<A, N> {
    Single(SingleOrderGeneric<A, N>),
    OneCancelsOther(OneCancelsOtherOrderGeneric<A, N>),
    OneTriggersOther(OneTriggersOtherOrderGeneric<A, N>),
    OneTriggersOco(OneTriggersOcoOrderGeneric<A, N>),
}

impl Resolvable<OrderRequest<AssetId, f64>> for OrderRequest<AssetIdValue, NumberValue> {
    fn try_resolve(
        &self,
        context: &ResolvedContext,
    ) -> StockTrekResult<OrderRequest<AssetId, f64>> {
        match self {
            Self::Single(order) => Ok(OrderRequest::Single(order.try_resolve(context)?)),
            Self::OneCancelsOther(order) => {
                Ok(OrderRequest::OneCancelsOther(order.try_resolve(context)?))
            }
            Self::OneTriggersOther(order) => {
                Ok(OrderRequest::OneTriggersOther(order.try_resolve(context)?))
            }
            Self::OneTriggersOco(order) => {
                Ok(OrderRequest::OneTriggersOco(order.try_resolve(context)?))
            }
        }
    }
}

impl<A, N> HasRequiredCapabilities for OrderRequest<A, N> {
    fn required_capabilities(&self) -> Vec<Capability> {
        match self {
            Self::Single(order) => order.required_capabilities(),
            Self::OneCancelsOther(order) => order.required_capabilities(),
            Self::OneTriggersOther(order) => order.required_capabilities(),
            Self::OneTriggersOco(order) => order.required_capabilities(),
        }
    }
}

impl OrderRequest<AssetId, f64> {
    pub fn into_precise(
        self,
        increments: &HashMap<TradingPair, IncrementSizes>,
        price_rounding: RoundingStrategy,
        quantity_rounding: RoundingStrategy,
        rate_rounding: RoundingStrategy,
    ) -> StockTrekResult<OrderRequest<AssetId, Decimal>> {
        match self {
            OrderRequest::OneCancelsOther(oco) => {
                let primary = oco.primary.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                let secondary = oco.secondary.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                let precise = OneCancelsOtherOrderGeneric { primary, secondary };
                Ok(OrderRequest::OneCancelsOther(precise))
            }
            OrderRequest::OneTriggersOther(oco) => {
                let primary = oco.primary.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                let secondary = oco.secondary.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                let precise = OneTriggersOtherOrderGeneric { primary, secondary };
                Ok(OrderRequest::OneTriggersOther(precise))
            }
            OrderRequest::OneTriggersOco(oco) => {
                let primary = oco.primary.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                let oco_primary = oco.oco_order.primary.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                let oco_secondary = oco.oco_order.secondary.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                let oco_order = OneCancelsOtherOrderGeneric {
                    primary: oco_primary,
                    secondary: oco_secondary,
                };
                let precise = OneTriggersOcoOrderGeneric { primary, oco_order };
                Ok(OrderRequest::OneTriggersOco(precise))
            }
            OrderRequest::Single(single) => {
                let precise = single.into_precise(
                    increments,
                    price_rounding,
                    quantity_rounding,
                    rate_rounding,
                )?;
                Ok(OrderRequest::Single(precise))
            }
        }
    }
}
