use crate::{
    error::result::StockTrekResult,
    execute::executor::{Executor, ExecutorTrait},
};
use serde_json::Value;

pub struct StubExecutor;

impl From<StubExecutor> for Executor {
    fn from(value: StubExecutor) -> Self {
        Box::new(value)
    }
}

impl ExecutorTrait for StubExecutor {
    fn send_message(&self, message: Value) -> StockTrekResult<Value> {
        Ok(message)
    }
}
