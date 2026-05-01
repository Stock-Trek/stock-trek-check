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
