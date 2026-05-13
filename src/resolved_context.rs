use crate::{
    execute::{exchanges::Exchanges, executor::Executor},
    order::check::order_check::OrderCheck,
    portfolios::portfolio::Portfolio,
    scratch::scratch_pad::ScratchPad,
};
use rust_decimal::RoundingStrategy;

pub struct ResolvedContext {
    pub price_rounding: RoundingStrategy,
    pub quantity_rounding: RoundingStrategy,
    pub rate_rounding: RoundingStrategy,
    pub order_checks: Vec<OrderCheck>,
    pub exchanges: Exchanges,
    pub executor: Executor,
    pub portfolio: Portfolio,
    pub scratch_pad: ScratchPad,
}
