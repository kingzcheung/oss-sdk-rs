//! UploadPartCopy 集成测试

use oss_sdk_rs::{Client, Config, Credentials};

mod common;

#[tokio::test]
async fn test_upload_part_copy_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client
        .upload_part_copy()
        .bucket("dest-bucket")
        .key("dest-object.zip")
        .upload_id("0004B9895DBBB6EC9****")
        .part_number(1)
        .source_bucket("src-bucket")
        .source_key("src-object.zip");
}

#[tokio::test]
async fn test_upload_part_copy_builder_with_range() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带拷贝范围的构建器
    let _builder = client
        .upload_part_copy()
        .bucket("dest-bucket")
        .key("dest-object.zip")
        .upload_id("upload-id")
        .part_number(1)
        .source_bucket("src-bucket")
        .source_key("src-object.zip")
        .copy_source_range_bytes(100, 6291756);
}

#[tokio::test]
async fn test_upload_part_copy_builder_with_version() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带版本 ID 的构建器
    let _builder = client
        .upload_part_copy()
        .bucket("dest-bucket")
        .key("dest-object.zip")
        .upload_id("upload-id")
        .part_number(1)
        .source_bucket("src-bucket")
        .source_key("src-object.zip")
        .source_version_id("version-123");
}

#[tokio::test]
async fn test_upload_part_copy_builder_with_conditions() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带条件拷贝的构建器
    let _builder = client
        .upload_part_copy()
        .bucket("dest-bucket")
        .key("dest-object.zip")
        .upload_id("upload-id")
        .part_number(1)
        .source_bucket("src-bucket")
        .source_key("src-object.zip")
        .copy_source_if_match("etag-123")
        .copy_source_if_unmodified_since("Fri, 13 Nov 2015 14:47:53 GMT");
}
