//! GetObjectMeta 集成测试

mod common;

use common::create_oss_client;

#[tokio::test]
async fn test_get_object_meta() {
    dotenvy::dotenv().ok();
    let client = create_oss_client();

    // 先上传一个对象
    let test_key = "test-get-object-meta.txt";
    let test_content = "Hello, GetObjectMeta test!";

    client
        .put_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .body(test_content.as_bytes().to_vec())
        .send()
        .await
        .expect("Failed to put object");

    // 获取对象元数据
    let output = client
        .get_object_meta()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .send()
        .await
        .expect("Failed to get object meta");

    // 验证元数据
    assert!(output.etag.is_some(), "ETag should be present");
    assert!(
        output.content_length.is_some(),
        "Content-Length should be present"
    );
    assert_eq!(output.content_length.unwrap(), test_content.len() as u64);
    assert!(
        output.last_modified.is_some(),
        "Last-Modified should be present"
    );
    assert!(output.request_id.is_some(), "Request-Id should be present");

    println!("ETag: {:?}", output.etag);
    println!("Content-Length: {:?}", output.content_length);
    println!("Last-Modified: {:?}", output.last_modified);
    println!("Request-ID: {:?}", output.request_id);

    // 清理测试对象
    client
        .delete_object()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key(test_key)
        .send()
        .await
        .expect("Failed to delete object");
}

#[tokio::test]
async fn test_get_object_meta_nonexistent() {
    dotenvy::dotenv().ok();
    let client = create_oss_client();

    // 尝试获取不存在的对象元数据
    let result = client
        .get_object_meta()
        .bucket(&std::env::var("OSS_BUCKET").unwrap())
        .key("nonexistent-object-meta.txt")
        .send()
        .await;

    assert!(result.is_err(), "Should fail for nonexistent object");
}
