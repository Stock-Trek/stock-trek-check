use crate::{
    error::result::StockTrekResult,
    execute::capability::{Capability, HasRequiredCapabilities},
    order::order_request::OrderRequest,
    resolved_context::ResolvedContext,
    resolvers::{
        resolveable::Resolvable,
        resolver::{Resolver, ResolverTrait},
    },
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnqueueOrderResolver {
    exchange_id_value: ExchangeIdValue,
    order_request: OrderRequest<AssetIdValue, NumberValue>,
}

impl EnqueueOrderResolver {
    pub fn new(
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
    ) -> Resolver {
        Box::new(Self {
            exchange_id_value,
            order_request,
        })
    }
}

#[typetag::serde]
impl ResolverTrait for EnqueueOrderResolver {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        let exchange_id = self.exchange_id_value.exchange_id(c)?;
        if let Some(adapter) = c.exchanges.adapter(&exchange_id) {
            let resolved_order_request = self.order_request.try_resolve(c)?;
            let mut precise_order_request = resolved_order_request.into_precise(
                adapter.increments(),
                c.price_rounding,
                c.quantity_rounding,
                c.rate_rounding,
            )?;
            for order_check in &c.order_checks {
                order_check.check(&mut precise_order_request)?;
            }
            if adapter.is_valid(&precise_order_request)? {
                adapter.enqueue_order(&precise_order_request, &c.executor)?;
                println!("enqueued order {:?}", precise_order_request);
            } else {
                println!("order is not valid {:?}", precise_order_request);
            }
        }
        Ok(())
    }
}

impl HasRequiredCapabilities for EnqueueOrderResolver {
    fn required_capabilities(&self) -> Vec<Capability> {
        self.order_request.required_capabilities()
    }
}
