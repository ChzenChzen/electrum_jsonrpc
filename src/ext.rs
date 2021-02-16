pub mod tests {
    use crate::ElectrumRpc;
    use std::env;
    use lazy_static::lazy_static;

    lazy_static!(
        pub static ref ADDR: String = if let Ok(var) = env::var("ELECTRUM_DAEMON_ADDRESS") {
                var
            } else {
                "http://127.0.0.1:7000".to_string()
            };


        pub static ref LOGIN: String = if let Ok(var) = env::var("ELECTRUM_USER") {
                var
            } else {
                "test".to_string()
            };

        pub static ref PASSWORD: String = if let Ok(var) = env::var("ELECTRUM_PASSWORD") {
                var
            } else {
                "test".to_string()
            };
    );



    pub fn get_electrum_rpc() -> ElectrumRpc {
        ElectrumRpc::new(
            LOGIN.clone(),
            PASSWORD.clone(),
            ADDR.clone(),
        ).unwrap()
    }
}