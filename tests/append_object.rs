//! AppendObject API 集成测试
//! 
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息

mod common;
use std::env;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 append_object API - 基本追加操作
#[tokio::test]
async fn test_append_object() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let key = "test/append_object_test.txt";
    
    // 清理可能存在的旧对象
    let _ = oss.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await;

    // 首次追加（position = 0）
    let content1 = b"Hello ".to_vec();
    let output = oss.append_object()
        .bucket(&bucket)
        .key(key)
        .position(0)
        .body(content1.clone())
        .content_type("text/plain")
        .send()
        .await?;

    println!("First append result:");
    println!("  ETag: {:?}", output.etag);
    println!("  Next append position: {:?}", output.next_append_position);
    println!("  CRC64: {:?}", output.hash_crc64ecma);

    assert!(output.next_append_position.is_some());
    let next_pos = output.next_append_position.unwrap();
    assert_eq!(next_pos, content1.len() as u64);

    // 第二次追加
    let content2 = b"World!".to_vec();
    let output = oss.append_object()
        .bucket(&bucket)
        .key(key)
        .position(next_pos)
        .body(content2.clone())
        .send()
        .await?;

    println!("\nSecond append result:");
    println!("  ETag: {:?}", output.etag);
    println!("  Next append position: {:?}", output.next_append_position);

    let expected_size = (content1.len() + content2.len()) as u64;
    assert_eq!(output.next_append_position, Some(expected_size));

    // 验证内容
    let get_output = oss.get_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    let body = get_output.body.collect().await
        .map_err(|e| OSSError::Io(e))?;
    let content = String::from_utf8(body.to_vec())
        .map_err(|e| OSSError::String(e))?;

    println!("\nFinal content: {}", content);
    assert_eq!(content, "Hello World!");

    // 清理
    oss.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}

/// 测试 append_object API - 多次追加
#[tokio::test]
async fn test_append_object_multiple() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let key = "test/append_object_multiple.txt";
    
    // 清理可能存在的旧对象
    let _ = oss.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await;

    // 多次追加
    let parts = vec!["Part1-", "Part2-", "Part3-", "Part4"];
    let mut current_pos: u64 = 0;

    for (i, part) in parts.iter().enumerate() {
        let output = oss.append_object()
            .bucket(&bucket)
            .key(key)
            .position(current_pos)
            .body(part.as_bytes().to_vec())
            .send()
            .await?;

        println!("Append {}: position={}, next_pos={:?}", 
                 i + 1, current_pos, output.next_append_position);

        current_pos = output.next_append_position.unwrap();
    }

    // 验证最终内容
    let get_output = oss.get_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    let body = get_output.body.collect().await
        .map_err(|e| OSSError::Io(e))?;
    let content = String::from_utf8(body.to_vec())
        .map_err(|e| OSSError::String(e))?;

    println!("Final content: {}", content);
    assert_eq!(content, "Part1-Part2-Part3-Part4");

    // 清理
    oss.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}

/// 测试 append_object API - 带元数据追加
#[tokio::test]
async fn test_append_object_with_metadata() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let key = "test/append_object_metadata.txt";
    
    // 清理可能存在的旧对象
    let _ = oss.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await;

    // 首次追加带元数据
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("author".to_string(), "test-user".to_string());
    metadata.insert("version".to_string(), "1.0".to_string());

    let output = oss.append_object()
        .bucket(&bucket)
        .key(key)
        .position(0)
        .body(b"test content".to_vec())
        .content_type("text/plain")
        .cache_control("no-cache")
        .metadata(metadata)
        .send()
        .await?;

    println!("Append with metadata result:");
    println!("  ETag: {:?}", output.etag);
    println!("  Next position: {:?}", output.next_append_position);

    // 清理
    oss.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}