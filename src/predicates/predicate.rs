use crate::{error::result::StockTrekResult, resolved_context::ResolvedContext};

pub type Predicate = Box<dyn PredicateTrait>;

#[typetag::serde]
pub trait PredicateTrait: Send + Sync {
    fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool>;
}
