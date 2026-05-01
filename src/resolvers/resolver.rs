use crate::{actions::action::Action, resolved_context::ResolvedContext};
use anyhow::Result;

pub type Resolver = Box<dyn ResolverTrait>;

#[typetag::serde]
pub trait ResolverTrait: Send + Sync {
    fn resolve(&self, context: &ResolvedContext, actions: &mut Vec<Action>) -> Result<()>;
}
