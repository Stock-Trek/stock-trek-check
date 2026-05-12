use crate::{
    asset_id::AssetId,
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    order::{
        check::order_check::OrderCheck, order_request::OrderRequest,
        orders::single::SingleOrderGeneric,
    },
};
use rust_decimal::Decimal;
use std::fmt::Display;

pub struct MultiLegCheck {
    pub if_different_symbol_unsupported: OnDifferent,
    pub if_different_price_unsupported: OnDifferent,
}

#[derive(Debug, Clone, Copy)]
pub enum OnDifferent {
    Error,
    PreferPrimary,
}

impl OrderCheck for MultiLegCheck {
    fn check(&self, order_request: &mut OrderRequest<AssetId, Decimal>) -> StockTrekResult<()> {
        match order_request {
            OrderRequest::Single(_) => Ok(()),
            OrderRequest::OneCancelsOther(oco) => {
                self.check_orders(&oco.primary, &mut oco.secondary)
            }
            OrderRequest::OneTriggersOther(oto) => {
                self.check_orders(&oto.primary, &mut oto.secondary)
            }
            OrderRequest::OneTriggersOco(otoco) => {
                self.check_orders(&otoco.primary, &mut otoco.oco_order.primary)?;
                self.check_orders(&otoco.primary, &mut otoco.oco_order.secondary)?;
                Ok(())
            }
        }?;
        Ok(())
    }
}

impl MultiLegCheck {
    fn check_orders(
        &self,
        primary: &SingleOrderGeneric<AssetId, Decimal>,
        secondary: &mut SingleOrderGeneric<AssetId, Decimal>,
    ) -> StockTrekResult<()> {
        Self::check_value(
            primary,
            secondary,
            |o| &o.base,
            |o, base| o.base = base,
            self.if_different_symbol_unsupported,
        )?;
        Self::check_value(
            primary,
            secondary,
            |o| &o.quote,
            |o, quote| o.quote = quote,
            self.if_different_symbol_unsupported,
        )?;
        Self::check_value(
            primary,
            secondary,
            |o| &o.pricing,
            |o, pricing| o.pricing = pricing,
            self.if_different_price_unsupported,
        )?;
        Ok(())
    }
    fn check_value<V, G, S>(
        primary: &SingleOrderGeneric<AssetId, Decimal>,
        secondary: &mut SingleOrderGeneric<AssetId, Decimal>,
        getter: G,
        setter: S,
        on_different: OnDifferent,
    ) -> StockTrekResult<()>
    where
        V: Eq + Clone + Display,
        G: Fn(&SingleOrderGeneric<AssetId, Decimal>) -> &V,
        S: Fn(&mut SingleOrderGeneric<AssetId, Decimal>, V),
    {
        let a_value = getter(primary);
        let b_value = getter(secondary);
        if a_value != b_value {
            match on_different {
                OnDifferent::Error => {
                    return Err(StockTrekError::Value(ValueError::ValuesNotEqual {
                        a: a_value.to_string(),
                        b: b_value.to_string(),
                    }))
                }
                OnDifferent::PreferPrimary => {
                    setter(secondary, a_value.clone());
                }
            }
        }
        Ok(())
    }
}
