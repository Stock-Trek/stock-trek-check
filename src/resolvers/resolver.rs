use crate::{
    error::result::StockTrekResult, execute::capability::HasRequiredCapabilities,
    resolved_context::ResolvedContext,
};

pub type Resolver = Box<dyn ResolverTrait>;

#[typetag::serde]
pub trait ResolverTrait: HasRequiredCapabilities + Send + Sync {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()>;
}
