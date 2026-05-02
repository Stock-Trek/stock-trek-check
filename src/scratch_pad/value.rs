use crate::error::{
    result::{StockTrekError, StockTrekResult},
    value::ValueError,
};
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScratchValue {
    Asset(Asset),
    Exchange(ExchangeId),
    Flag(bool),
    Number(f64),
}

impl Into<ScratchValue> for String {
    fn into(self) -> ScratchValue {
        ScratchValue::Asset(self)
    }
}
impl Into<ScratchValue> for ExchangeId {
    fn into(self) -> ScratchValue {
        ScratchValue::Exchange(self)
    }
}
impl Into<ScratchValue> for bool {
    fn into(self) -> ScratchValue {
        ScratchValue::Flag(self)
    }
}
impl Into<ScratchValue> for f64 {
    fn into(self) -> ScratchValue {
        ScratchValue::Number(self)
    }
}

impl TryFrom<ScratchValue> for ExchangeId {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(_) => err("Exchange", "Asset"),
            ScratchValue::Exchange(e) => Ok(e),
            ScratchValue::Flag(_) => err("Exchange", "Flag"),
            ScratchValue::Number(_) => err("Exchange", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for bool {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(_) => err("Flag", "Asset"),
            ScratchValue::Exchange(_) => err("Flag", "Exchange"),
            ScratchValue::Flag(f) => Ok(f),
            ScratchValue::Number(_) => err("Flag", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for f64 {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(_) => err("Number", "Asset"),
            ScratchValue::Exchange(_) => err("Number", "Exchange"),
            ScratchValue::Flag(_) => err("Number", "Flag"),
            ScratchValue::Number(n) => Ok(n),
        }
    }
}
impl TryFrom<ScratchValue> for Asset {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(a) => Ok(a),
            ScratchValue::Exchange(_) => err("Asset", "Exchange"),
            ScratchValue::Flag(_) => err("Asset", "Flag"),
            ScratchValue::Number(_) => err("Asset", "Number"),
        }
    }
}

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
