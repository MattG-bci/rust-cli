

#[cfg(test)]
mod tests {
    use rust_cli::api;
    use std::env;
    use serial_test::serial;

    #[should_panic]
    #[test]
    #[serial]
    fn test_obsidian_client_unsupported_client_type() {
        env::set_var("CLIENT_TYPE", "notion");
        api::client::Client::new();
    }

    #[test]
    #[serial]
    #[ignore]
    fn test_obsidian_client_supported_client_types() {
        let client_types: [&str; 2] = ["obsidian", "local"];
        for client_type in client_types {
            env::set_var("CLIENT_TYPE", client_type);
            api::client::Client::new();
        }
    }

}






