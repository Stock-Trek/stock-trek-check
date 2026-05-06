use crate::{
    error::result::StockTrekResult, order::order_time_in_force::OrderTimeInForce,
    resolved_context::ResolvedContext, resolvers::resolveable::Resolvable,
    values::value::NumberValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderPricing<P> {
    Market,
    Limit {
        price: P,
        time_in_force: OrderTimeInForce,
    },
}

impl Resolvable<OrderPricing<f64>> for OrderPricing<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OrderPricing<f64>> {
        match self {
            Self::Market => Ok(OrderPricing::Market),
            Self::Limit {
                price,
                time_in_force,
            } => Ok(OrderPricing::Limit {
                price: price.number(context)?,
                time_in_force: time_in_force.clone(),
            }),
        }
    }
}
