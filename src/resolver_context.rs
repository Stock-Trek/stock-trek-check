use crate::{
    actions::actions::Actions,
    predicates::predicates::Predicates,
    resolvers::resolvers::Resolvers,
    values::values::{CalculationValues, LiteralValues, PortfolioValues, ScratchPadValues},
};

pub struct ResolverContext {
    pub actions: Actions,
    pub calculations: CalculationValues,
    pub literals: LiteralValues,
    pub portfolio: PortfolioValues,
    pub predicates: Predicates,
    pub resolvers: Resolvers,
    pub scratch_pad: ScratchPadValues,
}
