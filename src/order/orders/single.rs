use std::collections::HashMap;

use crate::{
    asset_id::AssetId,
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    execute::{
        capability::{Capability, HasRequiredCapabilities},
        increment_sizes::IncrementSizes,
    },
    order::{
        order_activation::OrderActivation, order_constraint::OrderConstraint,
        order_intent::OrderIntent, order_pricing::OrderPricing, order_quantity::OrderQuantity,
        order_side::OrderSide, trading_pair::TradingPair,
    },
    resolved_context::ResolvedContext,
    resolvers::resolveable::Resolvable,
    values::value::{AssetIdValue, NumberValue},
};
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SingleOrderGeneric<A, N> {
    pub base: A,
    pub quote: A,
    pub activation: OrderActivation<N>,
    pub pricing: OrderPricing<N>,
    pub intent: OrderIntent,
    pub side: OrderSide,
    pub quantity: OrderQuantity<N>,
    pub constraints: Vec<OrderConstraint>,
}

pub type SingleOrderRaw = SingleOrderGeneric<AssetIdValue, NumberValue>;
pub type SingleOrder = SingleOrderGeneric<AssetId, f64>;

impl Resolvable<SingleOrder> for SingleOrderRaw {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<SingleOrder> {
        Ok(SingleOrder {
            base: self.base.asset_id(context)?,
            quote: self.quote.asset_id(context)?,
            activation: self.activation.try_resolve(context)?,
            pricing: self.pricing.try_resolve(context)?,
            intent: self.intent.clone(),
            side: self.side.clone(),
            quantity: self.quantity.try_resolve(context)?,
            constraints: self.constraints.clone(),
        })
    }
}

impl<A, N> HasRequiredCapabilities for SingleOrderGeneric<A, N> {
    fn required_capabilities(&self) -> Vec<Capability> {
        let mut capabilities = Vec::new();
        if let OrderQuantity::OfQuote(_) = self.quantity {
            if let OrderPricing::Limit { .. } = self.pricing {
                capabilities.push(Capability::QuoteQuantityWithLimitPricing);
            }
            match self.activation {
                OrderActivation::PriceTriggered { .. } => {
                    capabilities.push(Capability::QuoteQuantityWithTriggeredTiming);
                }
                OrderActivation::Trailing { .. } => {
                    capabilities.push(Capability::QuoteQuantityWithTriggeredTiming);
                }
                _ => {}
            }
        }
        capabilities
    }
}

impl SingleOrder {
    pub fn into_precise(
        self,
        increments: &HashMap<TradingPair, IncrementSizes>,
        price_rounding: RoundingStrategy,
        quantity_rounding: RoundingStrategy,
        rate_rounding: RoundingStrategy,
    ) -> StockTrekResult<SingleOrderGeneric<AssetId, Decimal>> {
        let SingleOrder {
            activation,
            base,
            constraints,
            intent,
            pricing,
            quantity,
            quote,
            side,
        } = self;
        let trading_pair_increments = increments
            .get(&TradingPair::new(base.clone(), quote.clone()))
            .ok_or_else(|| {
                StockTrekError::Value(ValueError::NotFound {
                    name: "Market".to_string(),
                    key: format!("Symbol({}/{})", base, quote),
                })
            })?;
        let activation: OrderActivation<Decimal> = match activation {
            OrderActivation::Immediate => OrderActivation::Immediate,
            OrderActivation::PriceTriggered {
                basis,
                direction,
                mode,
                price,
            } => OrderActivation::PriceTriggered {
                basis,
                direction,
                mode,
                price: trading_pair_increments.to_valid_tick(price, price_rounding),
            },
            OrderActivation::Trailing {
                activation_price,
                basis,
                callback_rate_bps,
                direction,
            } => OrderActivation::Trailing {
                activation_price: trading_pair_increments
                    .to_valid_tick(activation_price, price_rounding),
                basis,
                callback_rate_bps: IncrementSizes::to_valid_decimal(
                    callback_rate_bps,
                    Decimal::ONE,
                    rate_rounding,
                ),
                direction,
            },
        };
        let pricing: OrderPricing<Decimal> = match pricing {
            OrderPricing::Market => OrderPricing::Market,
            OrderPricing::Limit {
                price,
                time_in_force,
            } => OrderPricing::Limit {
                price: trading_pair_increments.to_valid_tick(price, price_rounding),
                time_in_force,
            },
        };
        let quantity: OrderQuantity<Decimal> = match quantity {
            // TODO this is probably wrong, uses same lot size for base and quote
            OrderQuantity::OfBase(q) => {
                OrderQuantity::OfBase(trading_pair_increments.to_valid_lot(q, quantity_rounding))
            }
            OrderQuantity::OfQuote(q) => {
                OrderQuantity::OfQuote(trading_pair_increments.to_valid_lot(q, quantity_rounding))
            }
        };
        Ok(SingleOrderGeneric::<AssetId, Decimal> {
            activation,
            base,
            constraints,
            intent,
            pricing,
            quantity,
            quote,
            side,
        })
    }
}
