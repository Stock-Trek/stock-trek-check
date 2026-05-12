use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderIntent {
    Open,
    Close { reduce_only: bool },
}
