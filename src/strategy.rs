use crate::{
    resolver_context::ResolverContext, resolvers::resolver::Resolver, scratch_pad::ScratchPad,
    strategy_context::StrategyContext,
};
use anyhow::Result;

pub trait Strategy: Send + Sync {
    fn market_calculations(&self, context: StrategyContext) -> Result<ScratchPad>;
    fn action_resolver(&self, context: ResolverContext) -> Result<Resolver>;
}
