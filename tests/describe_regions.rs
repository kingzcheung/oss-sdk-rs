//! DescribeRegions API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息

mod common;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 describe_regions API - 查询所有地域
#[tokio::test]
async fn test_describe_regions() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();

    // 查询所有地域
    let output = oss.describe_regions().send().await?;

    println!("Describe regions result:");
    println!("  Total regions: {}", output.region_info_list.len());

    for info in &output.region_info_list {
        println!("  - Region: {}", info.region);
        println!("    InternetEndpoint: {}", info.internet_endpoint);
        println!("    InternalEndpoint: {}", info.internal_endpoint);
        println!("    AccelerateEndpoint: {}", info.accelerate_endpoint);
    }

    assert!(
        !output.region_info_list.is_empty(),
        "Should have at least one region"
    );

    Ok(())
}

/// 测试 describe_regions API - 查询指定地域
#[tokio::test]
async fn test_describe_regions_with_region() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();

    // 查询指定地域
    let output = oss
        .describe_regions()
        .region("oss-cn-hangzhou")
        .send()
        .await?;

    println!("Describe regions result for oss-cn-hangzhou:");
    for info in &output.region_info_list {
        println!("  - Region: {}", info.region);
        println!("    InternetEndpoint: {}", info.internet_endpoint);
    }

    // 应该只返回一个地域
    assert!(
        !output.region_info_list.is_empty(),
        "Should have at least one region"
    );

    Ok(())
}
