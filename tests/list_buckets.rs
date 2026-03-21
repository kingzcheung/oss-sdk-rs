//! ListBuckets（GetService）API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息
//!
//! 环境变量：
//! - OSS_ACCESS_KEY_ID: 阿里云 AccessKey ID
//! - OSS_ACCESS_KEY_SECRET: 阿里云 AccessKey Secret
//! - OSS_ENDPOINT: OSS endpoint (如: https://oss-cn-hangzhou.aliyuncs.com)

mod common;
use oss_sdk_rs::errors::OSSError;

use common::*;

/// 测试 list_buckets API
#[tokio::test]
async fn test_list_buckets() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();

    let output = oss.list_buckets().max_keys(100).send().await?;

    println!("List buckets result:");
    println!(
        "  Owner: {} ({})",
        output.owner.display_name, output.owner.id
    );
    println!("  Is truncated: {:?}", output.is_truncated);

    if let Some(prefix) = &output.prefix {
        println!("  Prefix: {}", prefix);
    }
    if let Some(marker) = &output.marker {
        println!("  Marker: {}", marker);
    }
    if let Some(max_keys) = output.max_keys {
        println!("  Max keys: {}", max_keys);
    }
    if let Some(next_marker) = &output.next_marker {
        println!("  Next marker: {}", next_marker);
    }

    println!("  Buckets count: {}", output.buckets.bucket.len());
    for bucket in &output.buckets.bucket {
        println!("    - Name: {}", bucket.name);
        println!("      Location: {}", bucket.location);
        println!("      Creation Date: {}", bucket.creation_date);
    }

    // 验证返回结果
    assert!(!output.owner.id.is_empty(), "Owner ID should not be empty");
    assert!(
        !output.owner.display_name.is_empty(),
        "Owner display name should not be empty"
    );

    Ok(())
}

/// 测试 list_buckets 带前缀过滤
#[tokio::test]
async fn test_list_buckets_with_prefix() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();

    // 先获取所有 bucket，找到一个可用的前缀
    let all_buckets = oss.list_buckets().max_keys(100).send().await?;

    if all_buckets.buckets.bucket.is_empty() {
        println!("No buckets found, skipping prefix test");
        return Ok(());
    }

    // 使用第一个 bucket 名称的第一个字符作为前缀
    let first_bucket = &all_buckets.buckets.bucket[0];
    let prefix = first_bucket.name.chars().next().unwrap().to_string();

    let output = oss
        .list_buckets()
        .prefix(&prefix)
        .max_keys(10)
        .send()
        .await?;

    println!("List buckets with prefix '{}' result:", prefix);
    println!("  Prefix: {:?}", output.prefix);
    println!("  Buckets count: {}", output.buckets.bucket.len());

    // 验证返回的 bucket 都以指定前缀开头
    for bucket in &output.buckets.bucket {
        assert!(
            bucket.name.starts_with(&prefix),
            "Bucket name '{}' should start with prefix '{}'",
            bucket.name,
            prefix
        );
    }

    Ok(())
}

/// 测试 list_buckets 分页功能
#[tokio::test]
async fn test_list_buckets_pagination() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();

    // 先获取所有 bucket 的总数
    let all_buckets = oss.list_buckets().max_keys(1000).send().await?;

    let total_count = all_buckets.buckets.bucket.len();

    if total_count <= 1 {
        println!("Not enough buckets for pagination test, skipping");
        return Ok(());
    }

    // 使用 max_keys=1 进行分页测试
    let first_page = oss.list_buckets().max_keys(1).send().await?;

    println!("First page:");
    println!("  Buckets count: {}", first_page.buckets.bucket.len());
    println!("  Is truncated: {:?}", first_page.is_truncated);

    assert_eq!(
        first_page.buckets.bucket.len(),
        1,
        "Should return exactly 1 bucket"
    );

    // 如果有更多结果，测试使用 marker 获取下一页
    if first_page.is_truncated.unwrap_or(false) {
        if let Some(next_marker) = &first_page.next_marker {
            let second_page = oss
                .list_buckets()
                .marker(next_marker)
                .max_keys(1)
                .send()
                .await?;

            println!("Second page:");
            println!("  Buckets count: {}", second_page.buckets.bucket.len());
            println!("  Marker: {:?}", second_page.marker);

            // 验证两页返回的 bucket 不同
            if !second_page.buckets.bucket.is_empty() {
                assert_ne!(
                    first_page.buckets.bucket[0].name, second_page.buckets.bucket[0].name,
                    "Different pages should return different buckets"
                );
            }
        }
    }

    Ok(())
}
