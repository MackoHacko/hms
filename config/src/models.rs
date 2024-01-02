use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HmsConfig {
    pub snip_limit: i64,
}

impl HmsConfig {
    pub fn default() -> Self {
        Self { snip_limit: 10 }
    }
}

impl fmt::Display for HmsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configuration:\n - Snip Limit: {}", self.snip_limit)
    }
}
