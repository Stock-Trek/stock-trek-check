use serde::{Deserialize, Serialize};
use strum::Display;
use table_enum::table_enum;

table_enum! {
  #[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
  pub enum AssetId(default_ticker: &'static str) {
      Bitcoin("BTC"),
      BitcoinCash("BCH"),
      BNB("BNB"),
      Cardano("ADA"),
      Dogecoin("DOGE"),
      Ether("ETH"),
      Solana("SOL"),
      Tether("USDT"),
      USDCoin("USDC"),
      XRP("XRP"),
    }
}
