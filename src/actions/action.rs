use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    exchanges::exchange::Exchange,
};
use digdigdig3::{core::OrderRequest, ExchangeId};
use std::collections::HashMap;

pub type Action = Box<dyn ActionTrait>;

#[typetag::serde]
pub trait ActionTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn ActionTrait>;
    fn complete(&mut self, exchanges: &HashMap<ExchangeId, Exchange>) -> StockTrekResult<()>;
    fn try_place_order(
        &self,
        exchanges: &HashMap<ExchangeId, Exchange>,
        exchange_id: &ExchangeId,
        request: &OrderRequest,
    ) -> StockTrekResult<()> {
        if let Some(exchange) = exchanges.get(exchange_id) {
            let order = exchange.place_order(request)?;
            println!("{:?}", order);
            Ok(())
        } else {
            Err(StockTrekError::Value(ValueError::NotFound {
                name: "Exchange".to_string(),
                key: exchange_id.as_str().to_string(),
            }))
        }
    }
}

impl Clone for Box<dyn ActionTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
