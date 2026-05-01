pub mod actions;
pub mod example;
pub mod exchanges;
pub mod market_data;
pub mod portfolio;
pub mod predicates;
pub mod resolved_context;
pub mod resolver_context;
pub mod resolvers;
pub mod scratch_pad;
pub mod statistics;
pub mod strategy;
pub mod strategy_context;
pub mod util;
pub mod values;
pub mod verification;

pub mod prelude {
    pub use crate::{
        resolver_context::ResolverContext,
        resolvers::resolver::Resolver,
        scratch_pad::{ScratchPad, ScratchValue},
        strategy::Strategy,
        strategy_context::StrategyContext,
    };

    pub use digdigdig3::{Asset, ExchangeId, Symbol};

    pub use traitreg;
    pub use traitreg::register as register_strategy;

    pub type TimestampMillis = u64;
}
