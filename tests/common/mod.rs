//! Copyright The iFREEGROUP/oss-sdk-rs Authors
use std::env;

use oss_sdk_rs::{Client, Config, Credentials};

pub fn create_oss_client() -> Client {
    dotenvy::dotenv().unwrap();
    let key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let endpoint = env::var("OSS_ENDPOINT").unwrap();

    dbg!(&key_id, &key_secret, &endpoint);

    let credentials = Credentials::new(key_id, key_secret);

    let config = Config::builder()
        .endpoint(endpoint)
        .credentials(credentials)
        .build()
        .expect("Failed to build config");

    Client::from_config(config).expect("Failed to create client")
}
