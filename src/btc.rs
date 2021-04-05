use rust_decimal::prelude::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Represents btc address
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BtcAddress<'a> {
    #[serde(borrow)]
    pub address: &'a str,
}

// todo: address verification
impl<'a> BtcAddress<'a> {
    /// Create a new address from String
    pub fn new(address: &'a str) -> Self {
        Self { address }
    }
}

impl<'a> From<&BtcAddress<'a>> for String {
    fn from(address: &BtcAddress<'a>) -> Self {
        address.address.to_string()
    }
}

impl<'a> From<&BtcAddress<'a>> for Value {
    fn from(address: &BtcAddress<'a>) -> Self {
        json!(address.address)
    }
}
