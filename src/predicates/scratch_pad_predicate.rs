use crate::{
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScratchPadPredicate {
    key: String,
}

impl ScratchPadPredicate {
    pub fn new(key: String) -> Predicate {
        Box::new(Self { key })
    }
}

#[typetag::serde]
impl PredicateTrait for ScratchPadPredicate {
    fn test(&self, context: &ResolvedContext) -> StockTrekResult<bool> {
        context.scratch_pad.read_flag_required(&self.key)
    }
}
