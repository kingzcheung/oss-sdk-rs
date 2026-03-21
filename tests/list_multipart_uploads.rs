//! ListMultipartUploads 集成测试

use oss_sdk_rs::{Client, Config, Credentials};

mod common;

#[tokio::test]
async fn test_list_multipart_uploads_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client.list_multipart_uploads().bucket("my-bucket");
}

#[tokio::test]
async fn test_list_multipart_uploads_builder_with_options() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带选项的构建器
    let _builder = client
        .list_multipart_uploads()
        .bucket("my-bucket")
        .delimiter("/")
        .max_uploads(100)
        .prefix("photos/")
        .encoding_type("url");
}

#[tokio::test]
async fn test_list_multipart_uploads_builder_with_markers() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带分页参数的构建器
    let _builder = client
        .list_multipart_uploads()
        .bucket("my-bucket")
        .key_marker("previous-key")
        .upload_id_marker("previous-upload-id");
}