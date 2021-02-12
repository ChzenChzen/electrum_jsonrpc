mod error;

use hyper::{Client, Uri};
use std::net::{ToSocketAddrs, SocketAddr};
use hyper::client::HttpConnector;
use error::ElectrumRpcError;

pub struct ElectrumRpc {
    login: String,
    password: String,
    address: SocketAddr,
    client: Client<HttpConnector>,
}

enum ElectrumMethod {
    GetBalance,

}

impl ElectrumRpc {
    pub fn new(login: String, password: String, address: String) -> Result<Self, ElectrumRpcError> {
        let client = Client::new();
        let address = address.parse::<SocketAddr>()?;

        Ok(Self {
            login,
            password,
            address,
            client,
        })
    }

    // fn call_method(&self, method: ElectrumMethod, params: )
}

#[cfg(test)]
mod tests {
    use super::*;

    static LOGIN: &str = "LOGIN";
    static PASSWORD: &str = "PASSWORD";
    static HOST: &str = "127.0.0.1:7000";

    #[test]
    fn new_electrum_instance0() {
        let electrum = ElectrumRpc::new(
            LOGIN.to_string(),
            PASSWORD.to_string(),
            HOST.to_string(),
        ).unwrap();

        let port = electrum.address.port();
        assert_eq!(port, 7000);
        let host = electrum.address.ip().to_string();
        assert_eq!(host, "127.0.0.1");
    }

    #[test]
    #[should_panic]
    fn new_electrum_instance1() {
        let electrum = ElectrumRpc::new(
            LOGIN.to_string(),
            PASSWORD.to_string(),
            "127.0.0.1".to_string(),
        ).unwrap();
    }
}