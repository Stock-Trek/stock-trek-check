use crate::{
    actions::action::{Action, ActionTrait},
    error::result::StockTrekResult,
    exchanges::{client_order_id_builder::ClientOrderIdBuilder, exchange::Exchange},
};
use digdigdig3::{core::OrderRequest, ExchangeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderRequestAction {
    exchange: ExchangeId,
    order_request: OrderRequest,
}

impl OrderRequestAction {
    pub fn new(exchange: ExchangeId, order_request: OrderRequest) -> Action {
        Box::new(Self {
            exchange,
            order_request,
        })
    }
}

#[typetag::serde]
impl ActionTrait for OrderRequestAction {
    fn clone_box(&self) -> Box<dyn ActionTrait> {
        Box::new(self.clone())
    }
    fn complete(&mut self, exchanges: &HashMap<ExchangeId, Exchange>) -> StockTrekResult<()> {
        self.order_request.client_order_id = ClientOrderIdBuilder::from(&self.order_request);
        self.try_place_order(exchanges, &self.exchange, &self.order_request)
    }
}
