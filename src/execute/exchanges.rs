use crate::{exchange_id::ExchangeId, execute::exchange_adapter::ExchangeAdapter};
use std::collections::HashMap;

pub struct Exchanges {
    exchanges: HashMap<ExchangeId, ExchangeAdapter>,
}

impl Exchanges {
    pub fn new(exchanges: HashMap<ExchangeId, ExchangeAdapter>) -> Self {
        Self { exchanges }
    }
    pub fn adapter(&self, exchange_id: &ExchangeId) -> Option<&ExchangeAdapter> {
        self.exchanges.get(exchange_id)
    }
}
