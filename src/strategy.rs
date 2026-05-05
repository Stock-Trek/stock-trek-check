use crate::{
    error::result::StockTrekResult,
    resolver_context::ResolverContext,
    resolvers::{on_action_error::OnActionError, resolver::Resolver},
    scratch::scratch_pad::ScratchPad,
    strategy_context::StrategyContext,
};

pub trait Strategy: Send + Sync {
    fn market_calculations(&self, c: &StrategyContext) -> StockTrekResult<ScratchPad>;
    fn on_action_error(&self) -> OnActionError;
    fn action_resolver(&self, c: &ResolverContext) -> StockTrekResult<Resolver>;
}
