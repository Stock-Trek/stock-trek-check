use crate::{
    asset_id::AssetId, error::result::StockTrekResult, order::order_request::OrderRequest,
};
use rust_decimal::Decimal;

pub trait OrderCheck {
    fn check(&self, order_request: &mut OrderRequest<AssetId, Decimal>) -> StockTrekResult<()>;
}
