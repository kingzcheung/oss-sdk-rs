//! SealAppendObject API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息
//!
//! 注意：调用 SealAppendObject 接口需要先提交工单申请开通
//! 如果未开通，测试会跳过验证步骤

mod common;
use std::env;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 seal_append_object API
///
/// 注意：此功能需要先向阿里云提交工单申请开通
#[tokio::test]
async fn test_seal_append_object() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let key = "test/seal_append_object_test.txt";

    // 清理可能存在的旧对象
    let _ = oss.delete_object().bucket(&bucket).key(key).send().await;

    // 1. 首先创建一个 Appendable Object
    println!("1. Creating Appendable Object...");
    let content = b"Hello, this is a test content for seal operation.".to_vec();
    let append_output = oss
        .append_object()
        .bucket(&bucket)
        .key(key)
        .position(0)
        .body(content.clone())
        .content_type("text/plain")
        .send()
        .await?;

    println!("   Append result:");
    println!("     ETag: {:?}", append_output.etag);
    println!(
        "     Next position: {:?}",
        append_output.next_append_position
    );

    let final_position = append_output.next_append_position.unwrap();

    // 2. 封存 Object
    println!("\n2. Sealing Appendable Object...");
    let seal_result = oss
        .seal_append_object()
        .bucket(&bucket)
        .key(key)
        .position(final_position)
        .send()
        .await;

    match seal_result {
        Ok(seal_output) => {
            println!("   Seal result:");
            println!("     ETag: {:?}", seal_output.etag);
            println!("     Object type: {:?}", seal_output.object_type);
            println!("     Storage class: {:?}", seal_output.storage_class);
            println!("     Sealed time: {:?}", seal_output.sealed_time);
            println!("     Content length: {:?}", seal_output.content_length);
            println!("     Request ID: {:?}", seal_output.request_id);

            // 验证封存时间存在
            assert!(
                seal_output.sealed_time.is_some(),
                "Sealed time should be present"
            );

            // 3. 验证无法再追加内容
            println!("\n3. Verifying append is no longer possible...");
            let result = oss
                .append_object()
                .bucket(&bucket)
                .key(key)
                .position(final_position)
                .body(b"extra content".to_vec())
                .send()
                .await;

            // 应该失败，因为 Object 已经被封存
            assert!(result.is_err(), "Append should fail after seal");
            println!("   Append correctly failed after seal");
        }
        Err(OSSError::Object { message, .. }) if message.contains("OperationNotSupported") => {
            // 功能未开通，跳过测试
            println!("   SealAppendObject is not enabled for this bucket.");
            println!("   Please submit a ticket to Alibaba Cloud to enable this feature.");
            println!("   Skipping seal verification...");
        }
        Err(e) => return Err(e),
    }

    // 清理
    println!("\n4. Cleaning up...");
    oss.delete_object().bucket(&bucket).key(key).send().await?;
    println!("   Deleted");

    Ok(())
}

/// 测试 seal_append_object API - 位置不匹配错误
#[tokio::test]
async fn test_seal_append_object_wrong_position() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let key = "test/seal_append_object_wrong_pos.txt";

    // 清理可能存在的旧对象
    let _ = oss.delete_object().bucket(&bucket).key(key).send().await;

    // 创建 Appendable Object
    let content = b"Test content".to_vec();
    let append_output = oss
        .append_object()
        .bucket(&bucket)
        .key(key)
        .position(0)
        .body(content.clone())
        .send()
        .await?;

    let actual_position = append_output.next_append_position.unwrap();
    let wrong_position = actual_position + 100; // 错误的位置

    // 尝试用错误的位置封存
    let result = oss
        .seal_append_object()
        .bucket(&bucket)
        .key(key)
        .position(wrong_position)
        .send()
        .await;

    // 应该失败，因为位置不匹配
    assert!(result.is_err(), "Seal should fail with wrong position");
    println!("Seal correctly failed with wrong position");

    // 清理
    oss.delete_object().bucket(&bucket).key(key).send().await?;

    Ok(())
}
