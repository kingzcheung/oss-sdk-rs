use std::env;

use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();

    let access_key_id = env::var("OSS_AK").unwrap();
    let access_key_secret = env::var("OSS_SK").unwrap();
    let endpoint = env::var("ENDPOINT").unwrap();

    // 创建凭证
    let credentials = Credentials::new(access_key_id, access_key_secret);

    // 创建配置
    let config = Config::builder()
        .endpoint(endpoint)
        .credentials(credentials)
        .build()?;

    // 创建客户端
    let client = Client::from_config(config)?;

    // 列出所有 Bucket
    let output = client
        .list_buckets()
        .max_keys(100)
        .send()
        .await?;

    println!("List buckets success:");
    println!("  Owner: {} ({})", output.owner.display_name, output.owner.id);
    
    if output.buckets.bucket.is_empty() {
        println!("  No buckets found.");
    } else {
        println!("  Buckets:");
        for bucket in output.buckets.bucket {
            println!("    - Name: {}", bucket.name);
            println!("      Location: {}", bucket.location);
            println!("      Creation Date: {}", bucket.creation_date);
            if let Some(region) = bucket.region {
                println!("      Region: {}", region);
            }
            if let Some(storage_class) = bucket.storage_class {
                println!("      Storage Class: {}", storage_class);
            }
            if let Some(extranet_endpoint) = bucket.extranet_endpoint {
                println!("      Extranet Endpoint: {}", extranet_endpoint);
            }
            if let Some(intranet_endpoint) = bucket.intranet_endpoint {
                println!("      Intranet Endpoint: {}", intranet_endpoint);
            }
            if let Some(resource_group_id) = bucket.resource_group_id {
                println!("      Resource Group ID: {}", resource_group_id);
            }
        }
    }

    // 示例：使用前缀过滤
    println!("\n--- List buckets with prefix 'my' ---");
    let output_with_prefix = client
        .list_buckets()
        .prefix("my")
        .max_keys(10)
        .send()
        .await?;

    println!("Buckets with prefix 'my':");
    for bucket in output_with_prefix.buckets.bucket {
        println!("  - {}", bucket.name);
    }

    Ok(())
}