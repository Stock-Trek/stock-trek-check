use crate::{
    error::result::StockTrekResult,
    predicates::predicate::{Predicate, PredicateTrait},
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotPredicate {
    predicate: Predicate,
}

impl NotPredicate {
    pub fn new(predicate: Predicate) -> Predicate {
        Box::new(Self { predicate })
    }
}

#[typetag::serde]
impl PredicateTrait for NotPredicate {
    fn test(&self, context: &ResolvedContext) -> StockTrekResult<bool> {
        let test_result = self.predicate.test(context)?;
        Ok(!test_result)
    }
}
