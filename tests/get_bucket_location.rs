//! GetBucketLocation API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息

mod common;
use std::env;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 get_bucket_location API
#[tokio::test]
async fn test_get_bucket_location() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    let output = oss.get_bucket_location().bucket(&bucket).send().await?;

    println!("Get bucket location result:");
    println!("  Location: {}", output.location_constraint);

    assert!(
        !output.location_constraint.is_empty(),
        "Location should not be empty"
    );

    Ok(())
}
