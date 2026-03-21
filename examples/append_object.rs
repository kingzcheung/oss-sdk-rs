use std::env;

use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();

    let buffer = "test append object content";
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

    // 注意：AppendObject 操作需要特殊处理
    // 目前使用 put_object 作为示例
    let output = client
        .put_object()
        .bucket(&bucket)
        .key("test/append_object.txt")
        .body(buffer.as_bytes().to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("Append object success: {:?}", output);

    Ok(())
}