use crate::{
    error::result::StockTrekResult, resolved_context::ResolvedContext,
    resolvers::resolveable::Resolvable, values::value::NumberValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderQuantity<Q> {
    OfBase(Q),
    OfQuote(Q),
}

impl Resolvable<OrderQuantity<f64>> for OrderQuantity<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OrderQuantity<f64>> {
        match self {
            OrderQuantity::OfBase(q) => Ok(OrderQuantity::OfBase(q.number(context)?)),
            OrderQuantity::OfQuote(q) => Ok(OrderQuantity::OfQuote(q.number(context)?)),
        }
    }
}
