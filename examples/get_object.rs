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

    // 获取对象
    let output = client
        .get_object()
        .bucket(&bucket)
        .key("test/put_object.txt")
        .send()
        .await?;

    let body = output.body_into_vec().await?;
    println!(
        "Get object success, content: {:?}",
        String::from_utf8_lossy(&body)
    );

    Ok(())
}
