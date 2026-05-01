use crate::{
    actions::action::Action,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoOpResolver {}

impl NoOpResolver {
    pub fn new() -> Resolver {
        Box::new(Self {})
    }
}

#[typetag::serde]
impl ResolverTrait for NoOpResolver {
    fn resolve(&self, _: &ResolvedContext, _: &mut Vec<Action>) -> Result<()> {
        Ok(())
    }
}
