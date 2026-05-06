use crate::{
    error::result::StockTrekResult,
    order::single_order::SingleOrder,
    resolved_context::ResolvedContext,
    resolvers::resolveable::Resolvable,
    scratch::key::TokenName,
    values::value::{NumberValue, TokenValue},
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderRequest<T, N> {
    Single(SingleOrder<T, N>),
    OneCancelsTheOther {
        a: Box<OrderRequest<T, N>>,
        b: Box<OrderRequest<T, N>>,
    },
    OneTriggersTheOther {
        primary: Box<OrderRequest<T, N>>,
        secondary: Box<OrderRequest<T, N>>,
    },
}

impl Resolvable<OrderRequest<TokenName, f64>> for OrderRequest<TokenValue, NumberValue> {
    fn try_resolve(
        &self,
        context: &ResolvedContext,
    ) -> StockTrekResult<OrderRequest<TokenName, f64>> {
        match self {
            Self::Single(order) => Ok(OrderRequest::Single(order.try_resolve(context)?)),
            Self::OneCancelsTheOther { a, b } => Ok(OrderRequest::OneCancelsTheOther {
                a: Box::new(a.try_resolve(context)?),
                b: Box::new(b.try_resolve(context)?),
            }),
            Self::OneTriggersTheOther { primary, secondary } => {
                Ok(OrderRequest::OneTriggersTheOther {
                    primary: Box::new(primary.try_resolve(context)?),
                    secondary: Box::new(secondary.try_resolve(context)?),
                })
            }
        }
    }
}
