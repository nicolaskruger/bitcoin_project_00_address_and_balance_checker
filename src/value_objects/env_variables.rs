use std::env;

pub struct EnvVariables {
    blockchain_user: String,
    blockchain_password: String,
    blockchain_uri: String,
}

impl EnvVariables {
    pub fn new() -> Self {
        Self {
            blockchain_user: env::var("BLOCKCHAIN_USER").unwrap_or("bitcoinrpc".to_string()),
            blockchain_password: env::var("BLOCKCHAIN_PASSWORD")
                .unwrap_or("supersegredo".to_string()),
            blockchain_uri: env::var("BLOCKCHAIN_URI")
                .unwrap_or("http://127.0.0.1:18332".to_string()),
        }
    }

    pub fn blockchain_user(&self) -> String {
        self.blockchain_user.to_string()
    }
    pub fn blockchain_password(&self) -> String {
        self.blockchain_password.to_string()
    }
    pub fn blockchain_uri(&self) -> String {
        self.blockchain_uri.to_string()
    }
}
