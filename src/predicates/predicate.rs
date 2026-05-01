use crate::resolved_context::ResolvedContext;
use anyhow::Result;

pub type Predicate = Box<dyn PredicateTrait>;

#[typetag::serde]
pub trait PredicateTrait: Send + Sync {
    fn test(&self, context: &ResolvedContext) -> Result<bool>;
}
