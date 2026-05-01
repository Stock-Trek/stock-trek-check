use crate::values::{
    asset_in_exchange_value::AssetInExchangeValue,
    asset_total_value::AssetTotalValue,
    binary_calculation_value::{BinaryCalculationValue, BinaryOperator},
    cash_in_exchange_value::CashInExchangeValue,
    cash_total_value::CashTotalValue,
    literal_value::{
        LiteralAssetValue, LiteralExchangeValue, LiteralFlagValue, LiteralNumberValue,
    },
    scratch_pad_value::{
        ScratchPadAssetValue, ScratchPadExchangeValue, ScratchPadFlagValue, ScratchPadNumberValue,
    },
    unary_calculation_value::{UnaryCalculationValue, UnaryOperator},
    value::{AssetValue, ExchangeValue, FlagValue, NumberValue},
};
use digdigdig3::ExchangeId;

pub struct PortfolioValues {}
pub struct CalculationValues {}
pub struct LiteralValues {}
pub struct ScratchPadValues {}

impl PortfolioValues {
    pub fn asset_in_exchange(&self, exchange: ExchangeValue, asset: AssetValue) -> NumberValue {
        AssetInExchangeValue::new(exchange, asset)
    }
    pub fn asset_total(&self, asset: AssetValue) -> NumberValue {
        AssetTotalValue::new(asset)
    }
    pub fn cash_in_exchange(&self, exchange: ExchangeValue) -> NumberValue {
        CashInExchangeValue::new(exchange)
    }
    pub fn cash_total(&self) -> NumberValue {
        CashTotalValue::new()
    }
}

impl CalculationValues {
    pub fn binary(
        &self,
        left: NumberValue,
        operator: BinaryOperator,
        right: NumberValue,
    ) -> NumberValue {
        BinaryCalculationValue::new(left, operator, right)
    }
    pub fn unary(&self, number: NumberValue, operator: UnaryOperator) -> NumberValue {
        UnaryCalculationValue::new(number, operator)
    }
}

impl LiteralValues {
    pub fn asset(&self, literal: impl AsRef<str>) -> AssetValue {
        LiteralAssetValue::new(literal.as_ref().to_string())
    }
    pub fn exchange(&self, literal: ExchangeId) -> ExchangeValue {
        LiteralExchangeValue::new(literal)
    }
    pub fn flag(&self, literal: bool) -> FlagValue {
        LiteralFlagValue::new(literal)
    }
    pub fn number(&self, literal: f64) -> NumberValue {
        LiteralNumberValue::new(literal)
    }
}

impl ScratchPadValues {
    pub fn asset(&self, key: impl AsRef<str>) -> AssetValue {
        ScratchPadAssetValue::new(key.as_ref().to_string())
    }
    pub fn exchange(&self, key: impl AsRef<str>) -> ExchangeValue {
        ScratchPadExchangeValue::new(key.as_ref().to_string())
    }
    pub fn flag(&self, key: impl AsRef<str>) -> FlagValue {
        ScratchPadFlagValue::new(key.as_ref().to_string())
    }
    pub fn number(&self, key: impl AsRef<str>) -> NumberValue {
        ScratchPadNumberValue::new(key.as_ref().to_string())
    }
}
