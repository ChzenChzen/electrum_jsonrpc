//! Integration tests for Electrum's json-rpc calls.

use std::path::PathBuf;

use hyper::{body, Uri};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde_json::Value;
use tokio;

use electrum_jsonrpc::btc::BtcAddress;
use electrum_jsonrpc::ext::tests::*;

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
    let path = Some(PathBuf::from(
        "/home/electrum/.electrum/testnet/wallets/default_wallet",
    ));
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
    let addr = BtcAddress::new("tb1qncyt0k7dr2kspmrg3znqu4k808c09k385v38dn");
    let url = Some(Uri::from_static("http://127.0.0.1:8888/notify_data"));
    let res = electrum.notify(&addr, url).await.unwrap();
    let slice = body::to_bytes(res.into_body()).await.unwrap();

    let json: Value = serde_json::from_slice(&slice).unwrap();
    assert_eq!(json["result"], true)
}

#[tokio::test]
async fn call_method_notify_empty_url() {
    let electrum = get_electrum_rpc();
    let addr = BtcAddress::new("tb1qncyt0k7dr2kspmrg3znqu4k808c09k385v38dn");
    let res = electrum.notify(&addr, None).await.unwrap();
    let slice = body::to_bytes(res.into_body()).await.unwrap();

    let json: Value = serde_json::from_slice(&slice).unwrap();
    assert_eq!(json["result"], true, "\njson body is: {}", json)
}

#[tokio::test]
async fn call_method_restore_wallet() {
    let electrum = get_electrum_rpc();
    electrum.close_wallet().await.unwrap();

    let seed_phrase =
        "clever city snake tonight action output garbage gun upset raven pudding know";
    let res = electrum.restore_wallet(&seed_phrase).await.unwrap();
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

#[tokio::test]
async fn call_method_pay_to() {
    let electrum = get_electrum_rpc();
    let addr = BtcAddress::new("tb1qncyt0k7dr2kspmrg3znqu4k808c09k385v38dn");
    let amount = Decimal::from_f64(0.00001).unwrap();
    let res = electrum.pay_to(&addr, amount, None).await.unwrap();
    let slice = body::to_bytes(res).await.unwrap();

    let json: Value = serde_json::from_slice(&slice).unwrap();
    let expected = "02000000000101b58c5be9c9ce77a8bacd01779fdcfbf566a936a5b89482d1bc3114525ee5f3ea0000000000fdffffff02e8030000000000001600149e08b7dbcd1aad00ec6888a60e56c779f0f2da276022000000000000160014d272035ef819d6311231c06014aed5cfb100009e0247304402203db69d69b3fa76050b6c3276bc21bb834996f2c84c31c17c813beba01079705002202d864669f12db9939ea78a45e4c4a982cca68304fef25f334bd6cbbc9971bc9b012103815054ce939185772574ef569fe31b601d5bad48f48d5edaef194cded838c31ac40f1e00";
    assert_eq!(json["result"], expected, "\njson body is: {}", json);
}
