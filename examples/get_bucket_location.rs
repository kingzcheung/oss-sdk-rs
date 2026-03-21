//! GetBucketLocation 示例
//! 演示如何获取 Bucket 的位置信息

use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载配置
    let access_key_id = std::env::var("OSS_ACCESS_KEY_ID")?;
    let access_key_secret = std::env::var("OSS_ACCESS_KEY_SECRET")?;
    let endpoint = std::env::var("OSS_ENDPOINT")
        .unwrap_or_else(|_| "https://oss-cn-hangzhou.aliyuncs.com".to_string());
    let bucket = std::env::var("OSS_BUCKET").unwrap_or_else(|_| "my-bucket".to_string());

    // 创建凭证
    let credentials = Credentials::new(access_key_id, access_key_secret);

    // 创建配置
    let config = Config::builder()
        .credentials(credentials)
        .endpoint(endpoint)
        .region("cn-hangzhou")
        .build()?;

    // 创建客户端
    let client = Client::from_config(config)?;

    // 获取 Bucket 位置信息
    let output = client.get_bucket_location().bucket(&bucket).send().await?;

    println!("Bucket: {}", bucket);
    println!("Location: {}", output.location_constraint);

    Ok(())
}
