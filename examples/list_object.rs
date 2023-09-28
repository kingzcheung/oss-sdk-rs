use std::{env, collections::HashMap};

use oss_sdk_rs::{prelude::OSS, errors::OSSError, object::ObjectAPI};


#[tokio::main]
async fn main()->Result<(), OSSError>{
    dotenvy::dotenv().unwrap();

    let key_id = env::var("OSS_AK").unwrap();
    let key_secret = env::var("OSS_SK").unwrap();
    let endpoint = env::var("ENDPOINT").unwrap();
    let bucket = env::var("BUCKET").unwrap();
    let oss = OSS::new(
        key_id,
        key_secret,
        endpoint,
        bucket,
    );

    let mut resource: HashMap<&str, Option<&str>> = HashMap::new();
    resource.insert("prefix", Some("dataset/raw/anitnet_test/empty"));

    // use v1 api
    let res = oss.list_object(None::<HashMap<&str, &str>>, resource).await?;
    dbg!(res);

    // user v2 api
    let res = oss.list_object_v2(Some("dataset/raw/anitnet_test/empty"), None).await?;
    dbg!(res);

    Ok(())
}