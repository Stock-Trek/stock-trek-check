use crate::{
    actions::action::Action, error::result::StockTrekResult, resolved_context::ResolvedContext,
};

pub type Resolver = Box<dyn ResolverTrait>;

#[typetag::serde]
pub trait ResolverTrait: Send + Sync {
    fn resolve(&self, context: &ResolvedContext, actions: &mut Vec<Action>) -> StockTrekResult<()>;
}
