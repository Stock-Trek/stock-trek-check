use crate::{
    error::{
        portfolio::PortfolioError,
        result::{StockTrekError, StockTrekResult},
    },
    portfolios::portfolio::{Portfolio, PortfolioTrait},
};
use digdigdig3::{Asset, ExchangeId};
use std::collections::HashMap;

pub struct InMemoryPortfolio {
    exchange_assets: HashMap<ExchangeId, Assets>,
}

struct Assets {
    tokens: HashMap<Asset, f64>,
}

impl Assets {
    fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
}

impl InMemoryPortfolio {
    pub fn new() -> Portfolio {
        Box::new(Self {
            exchange_assets: HashMap::new(),
        })
    }
    pub fn add_tokens(
        &mut self,
        exchange_id: ExchangeId,
        asset: Asset,
        quantity: f64,
    ) -> StockTrekResult<()> {
        let exchange = self
            .exchange_assets
            .entry(exchange_id)
            .or_insert(Assets::new());
        exchange
            .tokens
            .entry(asset)
            .and_modify(|previous| *previous += quantity)
            .or_insert(quantity);
        Ok(())
    }
    pub fn remove_tokens(
        &mut self,
        exchange_id: ExchangeId,
        asset: Asset,
        quantity: f64,
    ) -> StockTrekResult<()> {
        let exchange = self.exchange_assets.get_mut(&exchange_id).ok_or_else(|| {
            StockTrekError::Portfolio(PortfolioError::NoAccountInExchange {
                exchange: exchange_id.as_str().to_string(),
            })
        })?;
        let token_quantity = exchange.tokens.get_mut(&asset).ok_or_else(|| {
            StockTrekError::Portfolio(PortfolioError::AssetNotOwned {
                exchange: exchange_id.as_str().to_string(),
                asset: asset.clone(),
            })
        })?;
        if *token_quantity < quantity {
            return Err(StockTrekError::Portfolio(PortfolioError::NotEnoughTokens {
                exchange: exchange_id.as_str().to_string(),
                asset,
                tokens: *token_quantity,
                remove_request: quantity,
            }));
        }
        *token_quantity -= quantity;
        if *token_quantity == 0.0 {
            exchange.tokens.remove(&asset);
        }
        Ok(())
    }
}

impl PortfolioTrait for InMemoryPortfolio {
    fn has_account_in_exchange(&self, exchange: ExchangeId) -> bool {
        self.exchange_assets.contains_key(&exchange)
    }
    fn owns_asset(&self, asset: &Asset) -> bool {
        self.exchange_assets
            .values()
            .any(|assets| assets.tokens.contains_key(asset))
    }
    fn owns_asset_in_exchange(&self, asset: &Asset, exchange: &ExchangeId) -> bool {
        self.exchange_assets
            .get(exchange)
            .map(|assets| assets.tokens.contains_key(asset))
            .unwrap_or(false)
    }
    fn asset_total(&self, asset: Asset) -> f64 {
        self.exchange_assets
            .values()
            .map(|assets| assets.tokens.get(&asset).unwrap_or(&0.0))
            .sum()
    }
    fn asset_in_exchange(&self, asset: Asset, exchange: ExchangeId) -> f64 {
        self.exchange_assets
            .get(&exchange)
            .and_then(|assets| assets.tokens.get(&asset))
            .copied()
            .unwrap_or(0.0)
    }
}
