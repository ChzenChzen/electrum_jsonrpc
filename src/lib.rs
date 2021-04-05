//! Simple asynchronous lib crate for interaction with Electrum client daemon via calling json-rpc methods.
//! Built on top of [tokio](https://docs.rs/tokio/1.2.0/tokio/) and [hyper](https://docs.rs/hyper/0.14.4/hyper/) crates.

use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str;

use base64;
use hyper::client::HttpConnector;
use hyper::header::AUTHORIZATION;
use hyper::{Body, Client, Method, Request, Response, Uri};
use log::info;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use btc::BtcAddress;
use error::Result;

pub mod btc;
pub mod error;
pub mod ext;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum ElectrumMethod {
    Broadcast,
    PayTo,
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

    ListAddresses,

    #[serde(rename = "list_requests")]
    ListRequests,

    Notify,
    Help,
    Empty,
    SignTransaction,

    #[serde(rename = "add_request")]
    AddRequest,
    #[serde(rename = "rmrequest")]
    RemoveRequest,
}

#[derive(Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Param {
    Text,

    #[serde(rename = "tx")]
    Transaction,

    #[serde(rename = "address")]
    BtcAddress,
    Destination,

    #[serde(rename = "wallet_path")]
    WalletPath,

    #[serde(rename = "URL")]
    Url,

    Password,
    Fee,
    Outputs,
    Amount,
    Memo,
    Pending,
    Expired,
    Paid,
}

struct JsonRpcBodyBuilder {
    json_rpc: f32,
    id: u64,
    method: ElectrumMethod,
    params: HashMap<Param, Value>,
}

impl JsonRpcBodyBuilder {
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

    pub fn add_param(mut self, param: Param, value: Value) -> Self {
        self.params.insert(param, value);
        self
    }

    pub fn build(self) -> JsonRpcBody {
        JsonRpcBody {
            json_rpc: self.json_rpc,
            id: self.id,
            method: self.method,
            params: self.params,
        }
    }
}

#[derive(Serialize)]
struct JsonRpcBody {
    json_rpc: f32,
    id: u64,
    method: ElectrumMethod,
    params: HashMap<Param, Value>,
}

impl JsonRpcBody {
    pub fn new() -> JsonRpcBodyBuilder {
        JsonRpcBodyBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice<'a> {
    amount: Decimal,

    #[serde(flatten, borrow)]
    address: BtcAddress<'a>,
}

impl<'a> Invoice<'a> {
    pub fn get_amount(&self) -> Decimal {
        self.amount
    }
    pub fn get_address(&self) -> &BtcAddress<'a> {
        &self.address
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

    async fn call_method(&self, body: &JsonRpcBody) -> Result<Response<Body>> {
        let payload = serde_json::to_string(body)?;
        info!("Payload is: {}", payload);

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
                .borrow(),
        )
        .await
    }

    /// Fetch the blockchain network info
    pub async fn get_info(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetInfo)
                .build()
                .borrow(),
        )
        .await
    }

    /// Return the balance of your wallet.
    pub async fn get_balance(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetBalance)
                .build()
                .borrow(),
        )
        .await
    }

    /// Return the transaction history of any address.
    /// Note: This is a walletless server query, results are not checked by SPV.
    pub async fn get_address_history<'a>(
        &self,
        address: &BtcAddress<'a>,
    ) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetAddressHistory)
                .add_param(Param::BtcAddress, Value::from(address))
                .build()
                .borrow(),
        )
        .await
    }

    /// Return the balance of any address.
    /// Note: This is a walletless server query, results are not checked by SPV.
    pub async fn get_address_balance<'a>(
        &self,
        address: &BtcAddress<'a>,
    ) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::GetAddressBalance)
                .add_param(Param::BtcAddress, Value::from(address))
                .build()
                .borrow(),
        )
        .await
    }

    /// List wallets opened in daemon
    pub async fn list_wallets(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::ListWallets)
                .build()
                .borrow(),
        )
        .await
    }

    /// Open wallet in daemon
    pub async fn load_wallet(
        &self,
        wallet_path: Option<PathBuf>,
        password: Option<&str>,
    ) -> Result<Response<Body>> {
        let mut builder = JsonRpcBody::new().method(ElectrumMethod::LoadWallet);

        if let Some(path) = &wallet_path {
            let path = path.to_str().unwrap();
            builder = builder.add_param(Param::WalletPath, Value::from(path))
        };

        if let Some(password) = password {
            builder = builder.add_param(Param::Password, Value::from(password))
        };

        self.call_method(&builder.build()).await
    }

    ///Create a new wallet
    pub async fn create_wallet(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::CreateWallet)
                .build()
                .borrow(),
        )
        .await
    }

    /// List wallet addresses.
    /// Returns the list of all addresses in your wallet.
    /// Use optional arguments to filter the results
    pub async fn list_addresses(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::ListAddresses)
                .build()
                .borrow(),
        )
        .await
    }
    /// Watch an address.
    /// Every time the address changes, a http POST is sent to the URL.
    /// Call with an `None` URL to stop watching an address.
    pub async fn notify<'a>(
        &self,
        address: &BtcAddress<'a>,
        url: Option<Uri>,
    ) -> Result<Response<Body>> {
        let url = url.unwrap_or(Uri::from_static("")).to_string();

        let builder = JsonRpcBody::new()
            .method(ElectrumMethod::Notify)
            .add_param(Param::BtcAddress, Value::from(address))
            .add_param(Param::Url, Value::from(url));

        self.call_method(&builder.build()).await
    }

    /// Restore a wallet from `text`. `text` can be a seed phrase, a master
    /// public key, a master private key, a list of bitcoin addresses
    /// or bitcoin private keys.
    pub async fn restore_wallet(&self, text: &str) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::RestoreWallet)
                .add_param(Param::Text, Value::from(text))
                .build()
                .borrow(),
        )
        .await
    }

    /// Sign a transaction. The wallet keys will be used unless a private key is provided.
    pub async fn sign_transaction(&self, tx: &str) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::SignTransaction)
                .add_param(Param::Transaction, Value::from(tx))
                .build()
                .borrow(),
        )
        .await
    }

    /// Broadcast a transaction to the network.
    pub async fn broadcast(&self, tx: &str) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::Broadcast)
                .add_param(Param::Transaction, Value::from(tx))
                .build()
                .borrow(),
        )
        .await
    }

    /// Create a transaction.
    pub async fn pay_to<'a>(
        &self,
        destination: &BtcAddress<'a>,
        amount: Decimal,
        fee: Option<Decimal>,
    ) -> Result<Response<Body>> {
        let mut builder = JsonRpcBody::new()
            .method(ElectrumMethod::PayTo)
            .add_param(Param::De, Value::from(destination))
            .add_param(Param::Amount, Value::from(amount.to_string()));

        if let Some(fee) = fee {
            builder = builder.add_param(Param::Fee, Value::from(fee.to_string()));
        }

        self.call_method(&builder.build()).await
    }

    /// Create a multi-output transaction.
    pub async fn pay_to_many(
        &self,
        fee: Decimal,
        outputs: Vec<(String, Decimal)>,
    ) -> Result<Response<Body>> {
        let outputs = json!(outputs);
        let fee = fee.to_string();
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::PayToMany)
                .add_param(Param::Fee, Value::from(fee))
                .add_param(Param::Outputs, outputs)
                .build()
                .borrow(),
        )
        .await
    }

    /// Close opened wallet.
    pub async fn close_wallet(&self) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::CloseWallet)
                .build()
                .borrow(),
        )
        .await
    }

    /// Create a payment request, using the first unused address of the wallet.
    /// The address will be considered as used after this operation.
    /// If no payment is received, the address will be considered as unused
    /// if the payment request is deleted from the wallet.
    pub async fn add_request(&self, amount: Decimal, memo: Option<&str>) -> Result<Response<Body>> {
        let amount = amount.to_string();

        let mut builder = JsonRpcBody::new()
            .method(ElectrumMethod::AddRequest)
            .add_param(Param::Amount, Value::from(amount.to_string()));

        if let Some(memo) = memo {
            builder = builder.add_param(Param::Memo, Value::from(memo))
        };

        self.call_method(&builder.build()).await
    }

    /// List the payment requests you made.
    /// You can combine `pending`, `expired` and `paid` flags for filtering.
    pub async fn list_requests(
        &self,
        pending: bool,
        expired: bool,
        paid: bool,
    ) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::ListRequests)
                .add_param(Param::Pending, Value::from(pending))
                .add_param(Param::Expired, Value::from(expired))
                .add_param(Param::Paid, Value::from(paid))
                .build()
                .borrow(),
        )
        .await
    }

    pub async fn remove_request<'a>(&self, address: &BtcAddress<'a>) -> Result<Response<Body>> {
        self.call_method(
            JsonRpcBody::new()
                .method(ElectrumMethod::RemoveRequest)
                .add_param(Param::BtcAddress, Value::from(address))
                .build()
                .borrow(),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{ElectrumRpcError, InvalidUri};
    use crate::ext::tests::*;

    use super::*;

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
        Electrum::new(LOGIN.clone(), PASSWORD.clone(), "".to_string()).unwrap();
    }

    #[test]
    fn error_casting_address_error() {
        let electrum = Electrum::new(LOGIN.clone(), PASSWORD.clone(), "".to_string());

        assert!(matches!(
            electrum,
            Err(ElectrumRpcError::AddressError(InvalidUri { .. }))
        ))
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
