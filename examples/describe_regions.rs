//! DescribeRegions 示例
//! 演示如何查询所有支持地域或指定地域对应的 Endpoint 信息

use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载配置
    let access_key_id = std::env::var("OSS_ACCESS_KEY_ID")?;
    let access_key_secret = std::env::var("OSS_ACCESS_KEY_SECRET")?;
    let endpoint = std::env::var("OSS_ENDPOINT")
        .unwrap_or_else(|_| "https://oss-cn-hangzhou.aliyuncs.com".to_string());

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

    // 示例 1: 查询所有支持地域对应的 Endpoint 信息
    println!("=== 查询所有支持地域 ===");
    let output = client.describe_regions().send().await?;

    println!("地域数量: {}", output.region_info_list.len());
    for info in &output.region_info_list {
        println!("  Region: {}", info.region);
        println!("    InternetEndpoint: {}", info.internet_endpoint);
        println!("    InternalEndpoint: {}", info.internal_endpoint);
        println!("    AccelerateEndpoint: {}", info.accelerate_endpoint);
    }

    // 示例 2: 查询指定地域对应的 Endpoint 信息
    println!("\n=== 查询指定地域 (oss-cn-hangzhou) ===");
    let output = client
        .describe_regions()
        .region("oss-cn-hangzhou")
        .send()
        .await?;

    println!("地域数量: {}", output.region_info_list.len());
    for info in &output.region_info_list {
        println!("  Region: {}", info.region);
        println!("    InternetEndpoint: {}", info.internet_endpoint);
        println!("    InternalEndpoint: {}", info.internal_endpoint);
        println!("    AccelerateEndpoint: {}", info.accelerate_endpoint);
    }

    Ok(())
}
