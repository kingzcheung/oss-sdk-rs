//! GetBucketStat API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息

mod common;
use std::env;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 get_bucket_stat API
#[tokio::test]
async fn test_get_bucket_stat() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    let output = oss.get_bucket_stat().bucket(&bucket).send().await?;

    println!("Get bucket stat result:");
    if let Some(storage) = output.storage {
        println!("  Storage: {} bytes", storage);
    }
    if let Some(count) = output.object_count {
        println!("  Object count: {}", count);
    }
    if let Some(count) = output.multipart_upload_count {
        println!("  Multipart upload count: {}", count);
    }
    if let Some(count) = output.live_channel_count {
        println!("  Live channel count: {}", count);
    }
    if let Some(time) = output.last_modified_time {
        println!("  Last modified time: {}", time);
    }

    // 标准存储
    println!("\n  Standard storage:");
    if let Some(storage) = output.standard_storage {
        println!("    Storage: {} bytes", storage);
    }
    if let Some(count) = output.standard_object_count {
        println!("    Object count: {}", count);
    }

    // 低频存储
    println!("\n  Infrequent access storage:");
    if let Some(storage) = output.infrequent_access_storage {
        println!("    Billing storage: {} bytes", storage);
    }
    if let Some(storage) = output.infrequent_access_real_storage {
        println!("    Real storage: {} bytes", storage);
    }
    if let Some(count) = output.infrequent_access_object_count {
        println!("    Object count: {}", count);
    }

    // 归档存储
    println!("\n  Archive storage:");
    if let Some(storage) = output.archive_storage {
        println!("    Billing storage: {} bytes", storage);
    }
    if let Some(storage) = output.archive_real_storage {
        println!("    Real storage: {} bytes", storage);
    }
    if let Some(count) = output.archive_object_count {
        println!("    Object count: {}", count);
    }

    // 验证基本字段存在
    assert!(
        output.storage.is_some() || output.object_count.is_some(),
        "Should have storage or object_count"
    );

    Ok(())
}
