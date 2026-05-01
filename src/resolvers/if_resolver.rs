use crate::{
    actions::action::Action,
    predicates::predicate::Predicate,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IfResolver {
    condition: Predicate,
    if_true: Resolver,
    if_false: Resolver,
}

impl IfResolver {
    pub fn new(condition: Predicate, if_true: Resolver, if_false: Resolver) -> Resolver {
        Box::new(Self {
            condition,
            if_true,
            if_false,
        })
    }
}

#[typetag::serde]
impl ResolverTrait for IfResolver {
    fn resolve(&self, context: &ResolvedContext, actions: &mut Vec<Action>) -> Result<()> {
        let predicate = self.condition.test(context)?;
        if predicate {
            self.if_true.resolve(context, actions)?;
        } else {
            self.if_false.resolve(context, actions)?;
        }
        Ok(())
    }
}
