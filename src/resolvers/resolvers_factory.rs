use crate::{
    order::order_request::OrderRequest,
    predicates::predicate::Predicate,
    resolvers::{
        enqueue_order_resolver::EnqueueOrderResolver, if_resolver::IfResolver,
        list_resolver::ListResolver, no_op_resolver::NoOpResolver, resolver::Resolver,
    },
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};

pub struct ResolversFactory;

impl ResolversFactory {
    pub fn if_else(&self, condition: Predicate, if_true: Resolver, if_false: Resolver) -> Resolver {
        IfResolver::new(condition, if_true, if_false)
    }
    pub fn list(&self, resolvers: Vec<Resolver>) -> Resolver {
        ListResolver::new(resolvers)
    }
    pub fn no_op(&self) -> Resolver {
        NoOpResolver::new()
    }
    pub fn enqueue_order(
        &self,
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
    ) -> Resolver {
        EnqueueOrderResolver::new(exchange_id_value, order_request)
    }
    // TODO
    // pub fn cancel_order(&self, exchange_id_value: ExchangeIdValue, order_id: OrderId) -> Resolver {
    //     CancelOrderResolver::new(exchange_id_value, order_id)
    // }
}
