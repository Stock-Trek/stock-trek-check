use crate::{
    error::result::StockTrekResult, order::order_trigger_direction::OrderTriggerDirection,
    resolved_context::ResolvedContext, resolvers::resolveable::Resolvable,
    values::value::NumberValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderTiming<N> {
    Immediate,
    Conditional {
        stop_price: N,
        trigger: OrderTriggerDirection,
    },
    Trailing {
        callback_rate: N,
        trigger: OrderTriggerDirection,
    },
}

impl Resolvable<OrderTiming<f64>> for OrderTiming<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OrderTiming<f64>> {
        match self {
            Self::Immediate => Ok(OrderTiming::Immediate),
            Self::Conditional {
                stop_price,
                trigger,
            } => Ok(OrderTiming::Conditional {
                stop_price: stop_price.number(context)?,
                trigger: trigger.clone(),
            }),
            Self::Trailing {
                callback_rate,
                trigger,
            } => Ok(OrderTiming::Trailing {
                callback_rate: callback_rate.number(context)?,
                trigger: trigger.clone(),
            }),
        }
    }
}
