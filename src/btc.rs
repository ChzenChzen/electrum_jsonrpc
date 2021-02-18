use serde::{Serialize, Deserialize};

/// Represents btc address
#[derive(Serialize, Deserialize, Debug)]
pub struct BtcAddress {
    address: String,
}


// todo: address verification
impl BtcAddress {
    /// Create a new address from String
    pub fn new(address: String) -> Self {
        Self { address }
    }
}


impl From<BtcAddress> for String {
    fn from(address: BtcAddress) -> Self {
        address.address
    }
}