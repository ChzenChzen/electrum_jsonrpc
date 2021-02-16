use electrum_jsonrpc::{ElectrumRpc};
use electrum_jsonrpc::ext::tests::*;
use tokio;
use std::path::Path;


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