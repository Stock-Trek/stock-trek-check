use crate::{
    actions::action::Action,
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListResolver {
    assemblers: Vec<Resolver>,
}

impl ListResolver {
    pub fn new(assemblers: Vec<Resolver>) -> Resolver {
        Box::new(Self { assemblers })
    }
}

#[typetag::serde]
impl ResolverTrait for ListResolver {
    fn resolve(&self, context: &ResolvedContext, actions: &mut Vec<Action>) -> StockTrekResult<()> {
        for assembler in &self.assemblers {
            assembler.resolve(context, actions)?;
        }
        Ok(())
    }
}
