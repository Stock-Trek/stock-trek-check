use crate::{exchange_id::ExchangeId, execute::exchange_adapter::ExchangeAdapter};
use std::collections::HashMap;

pub struct Exchanges {
    exchanges: HashMap<ExchangeId, Box<dyn ExchangeAdapter>>,
}

impl Exchanges {
    pub fn new(exchanges: HashMap<ExchangeId, Box<dyn ExchangeAdapter>>) -> Self {
        Self { exchanges }
    }
    pub fn adapter(&self, exchange_id: &ExchangeId) -> Option<&dyn ExchangeAdapter> {
        self.exchanges.get(exchange_id).map(|a| a.as_ref())
    }
}
