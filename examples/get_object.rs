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

    let object_name = "test.txt";
    let object_content = oss.get_object(object_name, None::<HashMap<&str, &str>>,None).await?;
    dbg!(object_content);

    Ok(())
}