use crate::{
    asset_id::AssetId, error::result::StockTrekResult, order::order_request::OrderRequest,
};
use rust_decimal::Decimal;

pub type OrderCheck = Box<dyn OrderCheckTrait>;

pub trait OrderCheckTrait {
    fn check(&self, order_request: &mut OrderRequest<AssetId, Decimal>) -> StockTrekResult<()>;
}
