use crate::{
    actions::actions_factory::ActionsFactory,
    predicates::predicates_factory::PredicatesFactory,
    resolvers::resolvers_factory::ResolversFactory,
    values::values_factory::{
        CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory,
        ScratchPadValuesFactory,
    },
};

pub struct ResolverContext {
    pub actions: ActionsFactory,
    pub calculations: CalculationValuesFactory,
    pub literals: LiteralValuesFactory,
    pub portfolio: PortfolioValuesFactory,
    pub predicates: PredicatesFactory,
    pub resolvers: ResolversFactory,
    pub scratch_pad: ScratchPadValuesFactory,
}

impl ResolverContext {
    pub fn new() -> Self {
        Self {
            actions: ActionsFactory {},
            calculations: CalculationValuesFactory {},
            literals: LiteralValuesFactory {},
            portfolio: PortfolioValuesFactory {},
            predicates: PredicatesFactory {},
            resolvers: ResolversFactory {},
            scratch_pad: ScratchPadValuesFactory {},
        }
    }
}

impl Default for ResolverContext {
    fn default() -> Self {
        Self::new()
    }
}
