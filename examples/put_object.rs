use std::{collections::HashMap, env};

use oss_sdk_rs::{errors::OSSError, prelude::OSS, object::ObjectAPI};

#[tokio::main]
async fn main()->Result<(), OSSError>{
    dotenvy::dotenv().unwrap();

    let buffer = "test async put object from buffer";
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
    let mut headers = HashMap::new();
    headers.insert("content-type", "text/plain");

    let mut oss_sub_resource: HashMap<&str, Option<&str>> = HashMap::new();
    oss_sub_resource.insert("acl", None);
    oss_sub_resource.insert("response-content-type", Some("ContentType"));

    oss.put_object(
            buffer.as_bytes(),
            "test/put_object.txt",
            headers,
            oss_sub_resource,
        )
        .await?;

    Ok(())
}