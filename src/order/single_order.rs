use crate::{
    error::result::StockTrekResult,
    order::{
        order_intent::OrderIntent, order_pricing::OrderPricing, order_quantity::OrderQuantity,
        order_side::OrderSide, order_timing::OrderTiming,
    },
    resolved_context::ResolvedContext,
    resolvers::resolveable::Resolvable,
    scratch::key::TokenName,
    values::value::{NumberValue, TokenValue},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SingleOrder<T, N> {
    pub base: T,
    pub quote: T,
    pub timing: OrderTiming<N>,
    pub pricing: OrderPricing<N>,
    pub intent: OrderIntent,
    pub side: OrderSide,
    pub quantity: OrderQuantity<N>,
}

impl Resolvable<SingleOrder<TokenName, f64>> for SingleOrder<TokenValue, NumberValue> {
    fn try_resolve(
        &self,
        context: &ResolvedContext,
    ) -> StockTrekResult<SingleOrder<TokenName, f64>> {
        Ok(SingleOrder {
            base: self.base.token(context)?,
            quote: self.quote.token(context)?,
            timing: self.timing.try_resolve(context)?,
            pricing: self.pricing.try_resolve(context)?,
            intent: self.intent.clone(),
            side: self.side.clone(),
            quantity: self.quantity.try_resolve(context)?,
        })
    }
}
