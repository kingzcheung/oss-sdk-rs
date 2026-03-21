//! AbortMultipartUpload 集成测试

use oss_sdk_rs::{Client, Config, Credentials};

mod common;

#[tokio::test]
async fn test_abort_multipart_upload_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client
        .abort_multipart_upload()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id-123");
}

#[tokio::test]
async fn test_abort_multipart_upload_builder_chained() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试链式调用
    let builder = client
        .abort_multipart_upload()
        .bucket("test-bucket")
        .key("test-object.dat")
        .upload_id("0004B9895DBBB6EC");

    // 验证构建器创建成功
    let _ = builder;
}