use crate::{
    error::result::StockTrekResult, predicates::predicate::PredicateTrait,
    resolved_context::ResolvedContext, scratch::key::ScratchKey,
};

#[typetag::serde]
impl PredicateTrait for ScratchKey<bool> {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        c.scratch_pad.read(self)
    }
}
