/// Represents btc address
pub struct BtcAddress {
    address: String,
}


// todo: address verification
impl BtcAddress {
    /// Create new address from String
    pub fn new(address: String) -> Self {
        Self { address }
    }
}


impl From<BtcAddress> for String {
    fn from(address: BtcAddress) -> Self {
        address.address
    }
}