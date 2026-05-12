use crate::{
    asset_id::AssetId,
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    exchange_id::ExchangeId,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum ScratchValue {
    ExchangeId(ExchangeId),
    AssetId(AssetId),
    Flag(bool),
    Number(f64),
}

impl From<AssetId> for ScratchValue {
    fn from(value: AssetId) -> Self {
        ScratchValue::AssetId(value)
    }
}
impl From<ExchangeId> for ScratchValue {
    fn from(value: ExchangeId) -> Self {
        ScratchValue::ExchangeId(value)
    }
}
impl From<bool> for ScratchValue {
    fn from(value: bool) -> Self {
        ScratchValue::Flag(value)
    }
}
impl From<f64> for ScratchValue {
    fn from(value: f64) -> Self {
        ScratchValue::Number(value)
    }
}

impl TryFrom<ScratchValue> for ExchangeId {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::ExchangeId(e) => Ok(e),
            ScratchValue::AssetId(_) => err("ExchangeId", "AssetId"),
            ScratchValue::Flag(_) => err("ExchangeId", "Flag"),
            ScratchValue::Number(_) => err("ExchangeId", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for AssetId {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::ExchangeId(_) => err("AssetId", "ExchangeId"),
            ScratchValue::AssetId(a) => Ok(a),
            ScratchValue::Flag(_) => err("AssetId", "Flag"),
            ScratchValue::Number(_) => err("AssetId", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for bool {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::ExchangeId(_) => err("Flag", "ExchangeId"),
            ScratchValue::AssetId(_) => err("Flag", "AssetId"),
            ScratchValue::Flag(f) => Ok(f),
            ScratchValue::Number(_) => err("Flag", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for f64 {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::ExchangeId(_) => err("Number", "ExchangeId"),
            ScratchValue::AssetId(_) => err("Number", "AssetId"),
            ScratchValue::Flag(_) => err("Number", "Flag"),
            ScratchValue::Number(n) => Ok(n),
        }
    }
}

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
