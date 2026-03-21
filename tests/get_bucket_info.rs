//! GetBucketInfo API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息

mod common;
use std::env;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 get_bucket_info API
#[tokio::test]
async fn test_get_bucket_info() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    let output = oss.get_bucket_info().bucket(&bucket).send().await?;

    println!("Get bucket info result:");
    println!("  Name: {}", output.bucket.name);
    println!("  Location: {}", output.bucket.location);
    println!("  Storage class: {}", output.bucket.storage_class);
    println!("  Creation date: {}", output.bucket.creation_date);
    println!("  Extranet endpoint: {}", output.bucket.extranet_endpoint);
    println!("  Intranet endpoint: {}", output.bucket.intranet_endpoint);
    println!("  Owner ID: {}", output.bucket.owner.id);
    println!("  ACL: {}", output.bucket.access_control_list.grant);

    assert_eq!(output.bucket.name, bucket);
    assert!(!output.bucket.location.is_empty());

    Ok(())
}
