//! Copyright The iFREEGROUP/oss-rust-sdk Authors
use std::env;

use oss_rust_sdk::prelude::OSS;

pub fn create_oss_client()->OSS<'static> {
    dotenvy::dotenv().unwrap();
    let key_id = env::var("OSS_AK").unwrap();
    let key_secret = env::var("OSS_SK").unwrap();
    let endpoint = env::var("ENDPOINT").unwrap();
    let bucket = env::var("BUCKET").unwrap();
    let oss_instance = OSS::new(
        key_id,
        key_secret,
        endpoint,
        bucket,
    );
    oss_instance
}