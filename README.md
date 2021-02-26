Simple library for interaction with Electron client daemon through json-rpc

```rust
 use electrum_jsonrpc::Electrum;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let client = Electrum::new(
        "dummy_login".to_string(),
        "dummy_password".to_string(),
        "http://127.0.0.1:7000".to_string(),
    )?;

    let resp = client.get_help().await?;

    Ok(())
}
```
