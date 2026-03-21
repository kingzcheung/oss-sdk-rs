//! PostObject 集成测试

mod common;

use common::create_oss_client;
use oss_sdk_rs::SuccessActionStatus;

#[tokio::test]
async fn test_post_object() {
    dotenvy::dotenv().ok();
    let client = create_oss_client();
    
    // 上传一个对象
    let test_key = "test-post-object.txt";
    let test_content = "Hello, PostObject test!";
    
    let output = client.post_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .body(test_content.as_bytes().to_vec())
        .content_type("text/plain")
        .send()
        .await
        .expect("Failed to post object");

    // 验证输出
    assert!(output.etag.is_some(), "ETag should be present");
    assert!(output.request_id.is_some(), "Request-Id should be present");
    assert!(output.bucket.is_some(), "Bucket should be present");
    assert!(output.key.is_some(), "Key should be present");

    println!("ETag: {:?}", output.etag);
    println!("Location: {:?}", output.location);
    println!("Request-ID: {:?}", output.request_id);

    // 清理测试对象
    client.delete_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .send()
        .await
        .expect("Failed to delete object");
}

#[tokio::test]
async fn test_post_object_with_metadata() {
    dotenvy::dotenv().ok();
    let client = create_oss_client();
    
    // 上传一个带元数据的对象
    let test_key = "test-post-object-metadata.txt";
    let test_content = "Hello, PostObject with metadata!";
    
    let output = client.post_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .body(test_content.as_bytes().to_vec())
        .content_type("text/plain")
        .success_action_status(SuccessActionStatus::Status200)
        .user_metadata("uuid", "test-uuid-123")
        .user_metadata("tag", "test-tag")
        .send()
        .await
        .expect("Failed to post object");

    // 验证输出
    assert!(output.etag.is_some(), "ETag should be present");

    println!("ETag: {:?}", output.etag);

    // 清理测试对象
    client.delete_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .send()
        .await
        .expect("Failed to delete object");
}

#[tokio::test]
async fn test_post_object_with_storage_class() {
    dotenvy::dotenv().ok();
    let client = create_oss_client();
    
    // 上传一个指定存储类型的对象
    let test_key = "test-post-object-storage-class.txt";
    let test_content = "Hello, PostObject with storage class!";
    
    let output = client.post_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .body(test_content.as_bytes().to_vec())
        .content_type("text/plain")
        .x_oss_storage_class("Standard")
        .send()
        .await
        .expect("Failed to post object");

    // 验证输出
    assert!(output.etag.is_some(), "ETag should be present");

    println!("ETag: {:?}", output.etag);

    // 清理测试对象
    client.delete_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .send()
        .await
        .expect("Failed to delete object");
}