use crate::{
    actions::action::Action,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ActionResolver {
    action: Action,
}

impl ActionResolver {
    pub fn new(action: Action) -> Resolver {
        Box::new(Self { action })
    }
}

#[typetag::serde]
impl ResolverTrait for ActionResolver {
    fn resolve(&self, _context: &ResolvedContext, actions: &mut Vec<Action>) -> Result<()> {
        actions.push(self.action.clone_box());
        Ok(())
    }
}
