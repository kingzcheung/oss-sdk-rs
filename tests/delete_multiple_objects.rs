//! DeleteMultipleObjects API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息

mod common;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 delete_multiple_objects API - 详细模式
#[tokio::test]
async fn test_delete_multiple_objects_verbose() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = std::env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    // 1. 先上传一些测试文件
    println!("1. Uploading test files...");
    let test_files: Vec<&str> = vec![
        "test/delete_multi_1.txt",
        "test/delete_multi_2.txt",
        "test/delete_multi_3.txt",
    ];

    for key in &test_files {
        let content = format!("Content of {}", key);
        oss.put_object()
            .bucket(&bucket)
            .key(*key)
            .body(content.into_bytes())
            .send()
            .await?;
        println!("   Uploaded: {}", key);
    }

    // 2. 批量删除文件（详细模式）
    println!("\n2. Deleting multiple objects (verbose mode)...");
    let output = oss
        .delete_multiple_objects()
        .bucket(&bucket)
        .object("test/delete_multi_1.txt")
        .object("test/delete_multi_2.txt")
        .object("test/delete_multi_3.txt")
        .quiet(false) // 详细模式
        .send()
        .await?;

    println!("   Delete result:");
    println!("     Request ID: {:?}", output.request_id);
    println!("     Deleted {} objects:", output.deleted.len());
    for deleted in &output.deleted {
        println!("       - {}", deleted.key);
    }

    // 验证删除成功
    assert_eq!(output.deleted.len(), 3, "Should have deleted 3 objects");

    // 3. 验证文件确实被删除
    println!("\n3. Verifying objects are deleted...");
    for key in &test_files {
        let result = oss.head_object().bucket(&bucket).key(*key).send().await;
        assert!(result.is_err(), "Object {} should be deleted", key);
        println!("   {} is deleted", key);
    }

    Ok(())
}

/// 测试 delete_multiple_objects API - 简单模式
#[tokio::test]
async fn test_delete_multiple_objects_quiet() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = std::env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    // 1. 先上传一些测试文件
    println!("1. Uploading test files...");
    let test_files: Vec<&str> = vec!["test/delete_quiet_1.txt", "test/delete_quiet_2.txt"];

    for key in &test_files {
        let content = format!("Content of {}", key);
        oss.put_object()
            .bucket(&bucket)
            .key(*key)
            .body(content.into_bytes())
            .send()
            .await?;
        println!("   Uploaded: {}", key);
    }

    // 2. 批量删除文件（简单模式）
    println!("\n2. Deleting multiple objects (quiet mode)...");
    let output = oss
        .delete_multiple_objects()
        .bucket(&bucket)
        .object("test/delete_quiet_1.txt")
        .object("test/delete_quiet_2.txt")
        .quiet(true) // 简单模式
        .send()
        .await?;

    println!("   Delete result:");
    println!("     Request ID: {:?}", output.request_id);
    println!(
        "     Deleted {} objects (quiet mode returns empty list)",
        output.deleted.len()
    );

    // 简单模式下，响应体为空，deleted 列表应该为空
    assert!(
        output.deleted.is_empty(),
        "Quiet mode should return empty deleted list"
    );

    // 3. 验证文件确实被删除
    println!("\n3. Verifying objects are deleted...");
    for key in &test_files {
        let result = oss.head_object().bucket(&bucket).key(*key).send().await;
        assert!(result.is_err(), "Object {} should be deleted", key);
        println!("   {} is deleted", key);
    }

    Ok(())
}

/// 测试 delete_multiple_objects API - 删除不存在的文件
#[tokio::test]
async fn test_delete_multiple_objects_nonexistent() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = std::env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    // 删除不存在的文件（OSS 不会报错，会返回成功）
    println!("Deleting non-existent objects...");
    let output = oss
        .delete_multiple_objects()
        .bucket(&bucket)
        .object("test/nonexistent_1.txt")
        .object("test/nonexistent_2.txt")
        .quiet(false)
        .send()
        .await?;

    println!("   Request ID: {:?}", output.request_id);
    println!("   Deleted {} objects", output.deleted.len());

    // OSS 删除不存在的文件也会返回成功
    // 但 deleted 列表可能为空或包含这些 key（取决于 OSS 实现）
    println!("   Note: OSS returns success even for non-existent objects");

    Ok(())
}

/// 测试 delete_multiple_objects API - 空对象列表错误
#[tokio::test]
async fn test_delete_multiple_objects_empty_list() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = std::env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    // 尝试删除空列表
    let result = oss
        .delete_multiple_objects()
        .bucket(&bucket)
        // 不添加任何对象
        .send()
        .await;

    // 应该返回错误
    assert!(result.is_err(), "Should fail with empty object list");
    println!("Correctly rejected empty object list");

    Ok(())
}

/// 测试 delete_multiple_objects API - 使用 ObjectIdentifier 列表
#[tokio::test]
async fn test_delete_multiple_objects_with_object_identifiers() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = std::env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    // 1. 先上传一些测试文件
    println!("1. Uploading test files...");
    use oss_sdk_rs::types::ObjectIdentifier;

    let test_files = vec![
        ObjectIdentifier::new("test/delete_objid_1.txt"),
        ObjectIdentifier::new("test/delete_objid_2.txt"),
    ];

    for obj in &test_files {
        let content = format!("Content of {}", obj.key);
        oss.put_object()
            .bucket(&bucket)
            .key(&obj.key)
            .body(content.into_bytes())
            .send()
            .await?;
        println!("   Uploaded: {}", obj.key);
    }

    // 2. 使用 objects 方法批量删除
    println!("\n2. Deleting using objects() method...");
    let output = oss
        .delete_multiple_objects()
        .bucket(&bucket)
        .objects(test_files.clone())
        .quiet(false)
        .send()
        .await?;

    println!("   Deleted {} objects", output.deleted.len());
    assert_eq!(output.deleted.len(), 2, "Should have deleted 2 objects");

    Ok(())
}
