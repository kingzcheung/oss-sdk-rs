//! UploadPart 集成测试

use oss_sdk_rs::{Client, Config, Credentials};

mod common;

#[tokio::test]
async fn test_upload_part_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client
        .upload_part()
        .bucket("test-bucket")
        .key("large-file.zip")
        .upload_id("0004B9895DBBB6EC9****")
        .part_number(1)
        .body(vec![1, 2, 3, 4, 5]);
}

#[tokio::test]
async fn test_upload_part_builder_with_large_part_number() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试最大分片号
    let _builder = client
        .upload_part()
        .bucket("test-bucket")
        .key("large-file.zip")
        .upload_id("upload-id")
        .part_number(10000)
        .body(vec![1, 2, 3, 4, 5]);
}

#[tokio::test]
async fn test_upload_part_builder_with_large_body() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试大数据
    let large_body = vec![0u8; 1024 * 1024]; // 1MB
    let _builder = client
        .upload_part()
        .bucket("test-bucket")
        .key("large-file.zip")
        .upload_id("upload-id")
        .part_number(1)
        .body(large_body);
}
