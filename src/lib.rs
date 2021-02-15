#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]

mod error;
mod ext;

use hyper::{Client, Uri, Request, Body, Method};
use std::net::{ToSocketAddrs, SocketAddr};
use hyper::client::HttpConnector;
use error::{ElectrumRpcError, Result};
use serde::{Serialize, Deserialize};
use hyper::body::{Bytes, Buf};
use hyper::header::AUTHORIZATION;
use base64;
use std::collections::HashMap;
use std::str;

pub struct ElectrumRpc {
    auth: String,
    address: Uri,
    client: Client<HttpConnector>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ElectrumMethod {
    #[serde(rename = "getinfo")]
    GetInfo,
    GetBalance,
    Help,
    Empty,
}


#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Param {}


struct RpcBodyBuilder {
    json_rpc: f32,
    id: u64,
    method: ElectrumMethod,
    params: HashMap<Param, String>,
}

impl RpcBodyBuilder {
    pub fn new() -> Self {
        Self {
            json_rpc: 2.0,
            id: 0,
            method: ElectrumMethod::Empty,
            params: HashMap::new(),
        }
    }

    pub fn json_rpc(mut self, value: f32) -> Self {
        self.json_rpc = value;
        self
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn method(mut self, method: ElectrumMethod) -> Self {
        self.method = method;
        self
    }

    pub fn param(mut self, key: Param, value: String) -> Self {
        self.params.insert(key, value);
        self
    }

    pub fn build(self) -> RpcBody {
        RpcBody {
            json_rpc: self.json_rpc,
            id: self.id,
            method: self.method,
            params: self.params,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RpcBody {
    json_rpc: f32,
    id: u64,
    method: ElectrumMethod,
    params: HashMap<Param, String>,
}

impl RpcBody {
    pub fn new() -> RpcBodyBuilder {
        RpcBodyBuilder::new()
    }
}


impl ElectrumRpc {
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

    async fn call_method(&self, body: &RpcBody) -> Result<String>
    {
        let req = Request::builder()
            .method(Method::POST)
            .header("accept", "application/json")
            .header(AUTHORIZATION, &self.auth)
            .uri("http://test:test@localhost:7000")
            .body(Body::from(serde_json::to_string(body).unwrap()))// serialize here!
            .unwrap();

        println!("{}", req.uri());

        let resp = self.client.request(req).await.unwrap();

        let buf = hyper::body::to_bytes(resp).await.unwrap();
        let text = str::from_utf8(buf.chunk()).unwrap();

        Ok(text.to_string())
    }

    pub async fn help(&self) -> Result<String> {
        let body = RpcBody::new()
            .json_rpc(2.0)
            .id(0)
            .method(ElectrumMethod::Help)
            .build();

        self.call_method(&body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::http::uri::{InvalidUri};
    use std::error::Error;

    static ADDR: &str = "http://127.0.0.1:7000";
    static LOGIN: &str = "test";
    static PASSWORD: &str = "test";

    fn get_electrum_rpc() -> ElectrumRpc {
        ElectrumRpc::new(
            LOGIN.to_string(),
            PASSWORD.to_string(),
            ADDR.to_string(),
        ).unwrap()
    }


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
        let electrum = ElectrumRpc::new(
            LOGIN.to_string(),
            PASSWORD.to_string(),
            "".to_string(),
        ).unwrap();
    }


    #[test]
    fn error_casting_address_error() {
        let electrum = ElectrumRpc::new(
            LOGIN.to_string(),
            PASSWORD.to_string(),
            "".to_string(),
        );

        assert!(matches!(electrum, Err(ElectrumRpcError::AddressError(InvalidUri))))
    }

    #[test]
    fn rpc_body() {
        let params = vec![].into_iter().collect();

        let body = RpcBody {
            json_rpc: 2.0,
            id: 1111,
            method: ElectrumMethod::GetInfo,
            params,
        };

        let actual = serde_json::to_string(&body).unwrap();
        let expected = r#"{"json_rpc":2.0,"id":1111,"method":"getinfo","params":{}}"#;
        assert_eq!(expected, actual);
    }

    // #[tokio::test]
    async fn call_method_simple_test() {
        let electrum = ElectrumRpc::new(
            LOGIN.to_string(),
            PASSWORD.to_string(),
            "http://127.0.0.1:7000/".to_string(),
        ).unwrap();

        let params = vec![].into_iter().collect();

        let body = RpcBody {
            json_rpc: 2.0,
            id: 1111,
            method: ElectrumMethod::GetInfo,
            params,
        };
        let res = electrum.call_method(&body).await.unwrap();
        println!("{}", res);
        assert_eq!("hello".to_string(), res);
    }
}