use serde::{Serialize, Deserialize};

/// Represents btc address
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BtcAddress<'a> {
    #[serde(borrow)]
    address: &'a str,
}


// todo: address verification
impl<'a> BtcAddress<'a> {
    /// Create a new address from String
    pub fn new(address: &'a str) -> Self {
        Self {
            address
        }
    }
}


impl<'a> From<&BtcAddress<'a>> for &'a str {
    fn from(address: &BtcAddress<'a>) -> Self {
        address.address
    }
}