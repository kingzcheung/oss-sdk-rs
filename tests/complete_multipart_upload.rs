//! CompleteMultipartUpload 集成测试

use oss_sdk_rs::{Client, Config, Credentials, ObjectAcl, PartItem};

mod common;

#[tokio::test]
async fn test_complete_multipart_upload_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client
        .complete_multipart_upload()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id")
        .part(1, "\"etag-1\"")
        .part(2, "\"etag-2\"");
}

#[tokio::test]
async fn test_complete_multipart_upload_builder_with_parts() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试使用 PartItem 列表
    let parts = vec![
        PartItem::new(1, "\"etag-1\""),
        PartItem::new(5, "\"etag-5\""),
        PartItem::new(8, "\"etag-8\""),
    ];

    let _builder = client
        .complete_multipart_upload()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id")
        .parts(parts);
}

#[tokio::test]
async fn test_complete_multipart_upload_builder_with_options() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带选项的构建器
    let _builder = client
        .complete_multipart_upload()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id")
        .part(1, "\"etag-1\"")
        .forbid_overwrite(true)
        .object_acl(ObjectAcl::Private)
        .encoding_type("url");
}

#[tokio::test]
async fn test_complete_multipart_upload_builder_complete_all() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试自动完成所有 Part
    let _builder = client
        .complete_multipart_upload()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id("upload-id")
        .complete_all(true);
}

#[test]
fn test_object_acl_variants() {
    assert_eq!(ObjectAcl::Default.to_string(), "default");
    assert_eq!(ObjectAcl::Private.to_string(), "private");
    assert_eq!(ObjectAcl::PublicRead.to_string(), "public-read");
    assert_eq!(ObjectAcl::PublicReadWrite.to_string(), "public-read-write");
}

#[test]
fn test_part_item_creation() {
    let part = PartItem::new(1, "\"etag-123\"");
    assert_eq!(part.part_number, 1);
    assert_eq!(part.etag, "\"etag-123\"");
}