use crate::error::result::StockTrekResult;
use serde_json::Value;

pub type Executor = Box<dyn ExecutorTrait>;

pub trait ExecutorTrait: Send + Sync {
    fn send_message(&self, message: Value) -> StockTrekResult<Value>;
}
