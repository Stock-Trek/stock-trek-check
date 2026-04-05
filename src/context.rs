use crate::{exchange::Exchange, statistics::stats::Stats};
use std::collections::HashMap;

#[derive(Clone)]
pub struct StockTrekContext {
    exchanges: HashMap<String, Exchange>,
    pub stats: Stats,
}

impl StockTrekContext {
    pub fn exchanges(&self) -> &HashMap<String, Exchange> {
        &self.exchanges
    }
}
