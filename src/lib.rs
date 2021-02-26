//! Simple asynchronous lib crate for interaction with Electrum client daemon via calling json-rpc methods.
//! Built on top of [tokio](https://docs.rs/tokio/1.2.0/tokio/) and [hyper](https://docs.rs/hyper/0.14.4/hyper/) crates.

pub mod btc;
pub mod error;
pub mod ext;

use base64;
use btc::BtcAddress;
use error::Result;
use hyper::client::HttpConnector;
use hyper::header::AUTHORIZATION;
use hyper::{Body, Client, Method, Request, Response, Uri};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str;
use log::info;
use rust_decimal::prelude::ToPrimitive;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum ElectrumMethod {
    PayToMany,

    #[serde(rename = "getinfo")]
    GetInfo,

    GetBalance,
    GetAddressHistory,
    GetAddressBalance,

    #[serde(rename = "list_wallets")]
    ListWallets,

    #[serde(rename = "close_wallet")]
    CloseWallet,

    #[serde(rename = "load_wallet")]
    LoadWallet,

    #[serde(rename = "create")]
    CreateWallet,

    #[serde(rename = "restore")]
    RestoreWallet,

    #[serde(rename = "listaddresses")]
    ListAddresses,

    Notify,
    Help,
    Empty,
}

#[derive(Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Param {
    Text,
    #[serde(rename = "address")]
    BtcAddress,
    #[serde(rename = "wallet_path")]
    WalletPath,
    #[serde(rename = "URL")]
    Url,
    Password,
}


struct JsonRpcBodyBuilder<'a> {
    json_rpc: f32,
    id: u64,
    method: ElectrumMethod,
    params: HashMap<Param, &'a str>,
}

impl<'a> JsonRpcBodyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            json_rpc: 2.0,
            id: 0,
            method: ElectrumMethod::Empty,
            params: HashMap::new(),
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn method(mut self, method: ElectrumMethod) -> Self {
        self.method = method;
        self
    }

    pub fn add_param(mut self, param: Param, value: &'a str) -> Self {
        self.params.insert(param, value);
        self
    }

    pub fn build(self) -> JsonRpcBody<'a> {
        JsonRpcBody {
            json_rpc: self.json_rpc,
            id: self.id,
            method: self.method,
            params: self.params,
        }
    }
}


#[derive(Serialize)]
struct JsonRpcBody<'a> {
    json_rpc: f32,
    id: u64,
    method: ElectrumMethod,
    params: HashMap<Param, &'a str>,
}

impl<'a> JsonRpcBody<'a> {
    pub fn new() -> JsonRpcBodyBuilder<'a> {
        JsonRpcBodyBuilder::new()
    }
}


/// Electrum JSON-RPC client.
///
/// Client represents methods for making json-rpc calls to Electrum daemon.
/// # Examples
/// ```
/// # use electrum_jsonrpc::Electrum;
/// # use hyper::{Response, Body};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Electrum::new(
///         "dummy_login".to_string(),
///         "dummy_password".to_string(),
///         "http://127.0.0.1:7000".to_string(),
///     )?;
///
///     let resp = client.get_help().await?;
///
///     Ok(())
/// }
/// ```


pub struct Electrum {
    auth: String,
    address: Uri,
    client: Client<HttpConnector>,
}

impl Electrum {
    /// Create new ElectrumRpc instance
    pub fn new(login: String, password: String, address: String) -> Result<Self> {
        let client = Client::new();
        let address = address.parse::<Uri>()?;
        let credentials = base64::encode(format!("{}:{}", login, password));
        let auth = format!("Basic {}", credentials);

        Ok(Self {
            auth,
            address,
            client,
        })
    }

    async fn call_method<'a>(&self, body: &'a JsonRpcBody<'a>) -> Result<Response<Body>> {
        let payload = serde_json::to_string(body)?;

        let req = Request::builder()
            .method(Method::POST)
            .header("accept", "application/json")
            .header(AUTHORIZATION, &self.auth)
            .uri(&self.address)
            .body(Body::from(payload))?;

        let resp = self.client.request(req).await?;

        Ok(resp)
    }

    /// List all available JSON-RPC calls
    pub async fn get_help(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .id(0)
                .method(ElectrumMethod::Help)
                .build()
                .borrow()
        ).await
    }

    /// Fetch the blockchain network info
    pub async fn get_info(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetInfo)
                .build()
                .borrow()
        ).await
    }

    /// Return the balance of your wallet.
    pub async fn get_balance(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetBalance)
                .build()
                .borrow()
        ).await
    }

    /// Return the transaction history of any address.
    /// Note: This is a walletless server query, results are not checked by SPV.
    pub async fn get_address_history<'a>(&self, address: &BtcAddress<'a>) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetAddressHistory)
                .add_param(Param::BtcAddress, address.into())
                .build()
                .borrow()
        ).await
    }

    /// Return the balance of any address.
    /// Note: This is a walletless server query, results are not checked by SPV.
    pub async fn get_address_balance<'a>(&self, address: &BtcAddress<'a>) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetAddressBalance)
                .add_param(Param::BtcAddress, address.into())
                .build()
                .borrow()
        ).await
    }

    /// List wallets opened in daemon
    pub async fn list_wallets(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::ListWallets)
                .build()
                .borrow()
        ).await
    }

    /// Open wallet in daemon
    pub async fn load_wallet(&self, wallet_path: Option<PathBuf>, password: Option<&str>) -> Result<Response<Body>> {
        let mut builder = JsonRpcBody::new()
            .method(ElectrumMethod::LoadWallet);

        if let Some(path) = &wallet_path {
            builder = builder.add_param(Param::WalletPath, path.to_str().unwrap())
        };

        if let Some(password) = password {
            builder = builder.add_param(Param::Password, password)
        };

        self.call_method(&builder.build()).await
    }

    ///Create a new wallet
    pub async fn create_wallet(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::CreateWallet)
                .build()
                .borrow()
        ).await
    }

    /// List wallet addresses.
    /// Returns the list of all addresses in your wallet.
    /// Use optional arguments to filter the results
    pub async fn list_addresses(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::ListAddresses)
                .build()
                .borrow()
        ).await
    }
    /// Watch an address.
    /// Every time the address changes, a http POST is sent to the URL.
    /// Call with an `None` URL to stop watching an address.
    pub async fn notify<'a>(&self, address: &BtcAddress<'a>, url: Option<Uri>) -> Result<Response<Body>> {
        let url = url
            .unwrap_or(Uri::from_static(""))
            .to_string();

        let builder = JsonRpcBody::new()
            .method(ElectrumMethod::Notify)
            .add_param(Param::BtcAddress, address.into())
            .add_param(Param::Url, &url);

        self.call_method(&builder.build()).await
    }

    /// Restore a wallet from `text`. `text` can be a seed phrase, a master
    /// public key, a master private key, a list of bitcoin addresses
    /// or bitcoin private keys.
    pub async fn restore_wallet(&self, text: &str) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::RestoreWallet)
                .add_param(Param::Text, text)
                .build()
                .borrow()
        ).await
    }


    /// Close opened wallet.
    pub async fn close_wallet(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::CloseWallet)
                .build()
                .borrow()
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{ElectrumRpcError, InvalidUri};
    use crate::ext::tests::*;

    #[test]
    fn new_electrum_instance0() {
        let electrum = get_electrum_rpc();
        let port = electrum.address.port();
        assert_eq!(port.unwrap().as_u16(), 7000);

        let host = electrum.address.host();
        assert_eq!(host, Some("127.0.0.1"));

        let encoded_creds = electrum.auth.split(' ').collect::<Vec<&str>>()[1];
        let decoded_creds = base64::decode(encoded_creds).unwrap();
        assert_eq!("test:test", std::str::from_utf8(&decoded_creds).unwrap());
    }

    #[test]
    #[should_panic]
    fn new_electrum_instance_empty_address() {
        Electrum::new(
            LOGIN.clone(),
            PASSWORD.clone(),
            "".to_string(),
        ).unwrap();
    }


    #[test]
    fn error_casting_address_error() {
        let electrum = Electrum::new(
            LOGIN.clone(),
            PASSWORD.clone(),
            "".to_string(),
        );

        assert!(matches!(electrum, Err(ElectrumRpcError::AddressError(InvalidUri {..}))))
    }

    #[test]
    fn rpc_body_builder() {
        let body = JsonRpcBody::new()
            .id(1111)
            .method(ElectrumMethod::GetInfo)
            .build();

        let actual = serde_json::to_string(&body).unwrap();
        let expected = r#"{"json_rpc":2.0,"id":1111,"method":"getinfo","params":{}}"#;
        assert_eq!(expected, actual);
    }
}