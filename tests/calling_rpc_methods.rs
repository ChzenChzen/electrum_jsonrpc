//! Integration tests for Electrum's json-rpc calls.

use electrum_jsonrpc::ext::tests::*;
use tokio;
use std::path::Path;
use electrum_jsonrpc::btc::BtcAddress;
use hyper::{Uri, body};
use serde_json::Value;


#[tokio::test]
async fn call_method_help() {
    let electrum = get_electrum_rpc();
    let res = electrum.get_help().await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn call_method_get_info() {
    let electrum = get_electrum_rpc();

    let res = electrum.get_info().await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn call_method_get_balance() {
    let electrum = get_electrum_rpc();

    let res = electrum.get_balance().await.unwrap();
    assert_eq!(res.status(), 200);
}


#[tokio::test]
async fn call_method_list_wallets() {
    let electrum = get_electrum_rpc();

    let res = electrum.list_wallets().await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn call_method_load_wallet_default_wallet() {
    let electrum = get_electrum_rpc();

    let res = electrum.load_wallet(None, None).await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn call_method_load_wallet_from_path_without_password() {
    let electrum = get_electrum_rpc();
    let path = Some(Box::from(Path::new("/home/electrum/.electrum/testnet/wallets/default_wallet")));
    let res = electrum.load_wallet(path, None).await.unwrap();
    assert_eq!(res.status(), 200);
}

// #[tokio::test]
// async fn call_method_load_wallet_from_path_with_password() {
//     let electrum = get_electrum_rpc();
//     let path = Some(Box::from(Path::new("/home/electrum/.electrum/testnet/wallets/default_wallet")));
//     let res = electrum.load_wallet(path, None).await.unwrap();
//     assert_eq!("hello".to_string(), res);
// }

#[tokio::test]
async fn call_method_create_wallet_default() {
    let electrum = get_electrum_rpc();
    let res = electrum.create_wallet().await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn call_method_list_addresses_default() {
    let electrum = get_electrum_rpc();
    let res = electrum.list_addresses().await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn call_method_notify_url() {
    let electrum = get_electrum_rpc();
    let addr = BtcAddress::new("tb1qncyt0k7dr2kspmrg3znqu4k808c09k385v38dn".to_string());
    let url = Some(Uri::from_static("http://127.0.0.1:8888/notify_data"));
    let res = electrum.notify(addr, url).await.unwrap();
    let slice = body::to_bytes(res.into_body()).await.unwrap();

    let json: Value = serde_json::from_slice(&slice).unwrap();
    assert_eq!(json["result"], true)
}

#[tokio::test]
async fn call_method_notify_empty_url() {
    let electrum = get_electrum_rpc();
    let addr = BtcAddress::new("tb1qncyt0k7dr2kspmrg3znqu4k808c09k385v38dn".to_string());
    let res = electrum.notify(addr, None).await.unwrap();
    let slice = body::to_bytes(res.into_body()).await.unwrap();

    let json: Value = serde_json::from_slice(&slice).unwrap();
    assert_eq!(json["result"], true, "\njson body is: {}", json)
}


#[tokio::test]
async fn call_method_restore_wallet() {
    let electrum = get_electrum_rpc();
    electrum.close_wallet().await.unwrap();

    let seed_phrase = "clever city snake tonight action output garbage gun upset raven pudding know".to_string();
    let res = electrum.restore_wallet(seed_phrase).await.unwrap();
    let slice = body::to_bytes(res).await.unwrap();

    let json: Value = serde_json::from_slice(&slice).unwrap();
    assert_eq!(json["result"], true, "\njson body is: {}", json);
}

#[tokio::test]
async fn call_method_close_wallet() {
    let electrum = get_electrum_rpc();
    let res = electrum.close_wallet().await.unwrap();
    let slice = body::to_bytes(res).await.unwrap();

    let json: Value = serde_json::from_slice(&slice).unwrap();
    assert_eq!(json["result"], true, "\njson body is: {}", json);
}

