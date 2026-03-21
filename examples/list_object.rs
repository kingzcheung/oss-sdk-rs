use std::env;

use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();

    let access_key_id = env::var("OSS_AK").unwrap();
    let access_key_secret = env::var("OSS_SK").unwrap();
    let endpoint = env::var("ENDPOINT").unwrap();
    let bucket = env::var("BUCKET").unwrap();

    // 创建凭证
    let credentials = Credentials::new(access_key_id, access_key_secret);

    // 创建配置
    let config = Config::builder()
        .endpoint(endpoint)
        .credentials(credentials)
        .build()?;

    // 创建客户端
    let client = Client::from_config(config)?;

    // 列出对象
    let output = client
        .list_objects()
        .bucket(&bucket)
        .prefix("test/")
        .delimiter("/")
        .max_keys(100)
        .send()
        .await?;

    println!("List objects success:");
    println!("  Bucket: {}", output.name);
    println!("  Is truncated: {}", output.is_truncated);

    if let Some(contents) = output.contents {
        for object in contents {
            println!("  - Key: {}, Size: {}", object.key, object.size);
        }
    }

    Ok(())
}
