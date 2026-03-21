//! ListParts 集成测试

use oss_sdk_rs::{Client, Config, Credentials};

mod common;

#[tokio::test]
async fn test_list_parts_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client
        .list_parts()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id-123");
}

#[tokio::test]
async fn test_list_parts_builder_with_options() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带选项的构建器
    let _builder = client
        .list_parts()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id-123")
        .max_parts(100)
        .part_number_marker(10)
        .encoding_type("url");
}

#[tokio::test]
async fn test_list_parts_builder_with_pagination() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带分页参数的构建器
    let _builder = client
        .list_parts()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id-123")
        .max_parts(50)
        .part_number_marker(25);
}
