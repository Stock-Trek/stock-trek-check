use crate::error::{
    result::{StockTrekError, StockTrekResult},
    value::ValueError,
};
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScratchValue {
    Asset(Asset),
    Exchange(ExchangeId),
    Flag(bool),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScratchPad {
    values: HashMap<String, ScratchValue>,
}

impl Default for ScratchPad {
    fn default() -> Self {
        Self::new()
    }
}

impl ScratchPad {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn write_asset(&mut self, key: impl AsRef<str>, asset: impl AsRef<str>) {
        self.write(key, ScratchValue::Asset(asset.as_ref().to_string()))
    }
    pub fn write_exchange(&mut self, key: impl AsRef<str>, exchange: ExchangeId) {
        self.write(key, ScratchValue::Exchange(exchange))
    }
    pub fn write_flag(&mut self, key: impl AsRef<str>, flag: bool) {
        self.write(key, ScratchValue::Flag(flag))
    }
    pub fn write_number(&mut self, key: impl AsRef<str>, number: f64) {
        self.write(key, ScratchValue::Number(number))
    }
    pub fn read_asset_required(&self, key: impl AsRef<str>) -> StockTrekResult<Asset> {
        self.read_required::<Asset>(key)
    }
    pub fn read_exchange_required(&self, key: impl AsRef<str>) -> StockTrekResult<ExchangeId> {
        self.read_required::<ExchangeId>(key)
    }
    pub fn read_flag_required(&self, key: impl AsRef<str>) -> StockTrekResult<bool> {
        self.read_required::<bool>(key)
    }
    pub fn read_number_required(&self, key: impl AsRef<str>) -> StockTrekResult<f64> {
        self.read_required::<f64>(key)
    }
    pub fn read_asset_optional(&self, key: impl AsRef<str>) -> Option<Asset> {
        self.read_optional::<Asset>(key)
    }
    pub fn read_exchange_optional(&self, key: impl AsRef<str>) -> Option<ExchangeId> {
        self.read_optional::<ExchangeId>(key)
    }
    pub fn read_flag_optional(&self, key: impl AsRef<str>) -> Option<bool> {
        self.read_optional::<bool>(key)
    }
    pub fn read_number_optional(&self, key: impl AsRef<str>) -> Option<f64> {
        self.read_optional::<f64>(key)
    }
}

impl ScratchPad {
    pub fn write(&mut self, key: impl AsRef<str>, value: ScratchValue) {
        self.values.insert(key.as_ref().to_string(), value);
    }
    fn read_required<T>(&self, key: impl AsRef<str>) -> StockTrekResult<T>
    where
        T: TryFrom<ScratchValue, Error = StockTrekError>,
    {
        let value = self.values.get(key.as_ref());
        match value {
            None => Err(StockTrekError::Value(ValueError::NotFound {
                name: "Key".to_string(),
                key: key.as_ref().to_string(),
            })),
            Some(v) => T::try_from(v.clone()),
        }
    }
    fn read_optional<T>(&self, key: impl AsRef<str>) -> Option<T>
    where
        T: TryFrom<ScratchValue, Error = StockTrekError>,
    {
        let value = self.values.get(key.as_ref());
        match value {
            None => None,
            Some(v) => T::try_from(v.clone()).ok(),
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

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
