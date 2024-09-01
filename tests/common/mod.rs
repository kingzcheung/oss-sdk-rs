//! Copyright The iFREEGROUP/oss-sdk-rs Authors
use std::env;

use oss_sdk_rs::{client::Client, config::{Config, ConfigBuilder}};


pub fn create_oss_client()->Client {
    dotenvy::dotenv().unwrap();
    let key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let endpoint = env::var("OSS_ENDPOINT").unwrap();
    let bucket = env::var("OSS_BUCKET").unwrap();

    dbg!(&key_id, &key_secret, &endpoint, &bucket);

    let conf = ConfigBuilder::builder()
    .set_access_key_id(key_id)
    .set_access_key_secret(key_secret)
    .set_endpoint(endpoint)
    .set_region("cn-guangzhou")
    .build();

    
    Client::from_conf(conf)
}