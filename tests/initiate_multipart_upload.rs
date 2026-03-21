//! InitiateMultipartUpload 集成测试

use oss_sdk_rs::{Client, Config, Credentials, ServerSideEncryption, StorageClass};

mod common;

#[tokio::test]
async fn test_initiate_multipart_upload_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client
        .initiate_multipart_upload()
        .bucket("test-bucket")
        .key("large-file.zip");
}

#[tokio::test]
async fn test_initiate_multipart_upload_builder_with_options() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带选项的构建器可以创建
    let _builder = client
        .initiate_multipart_upload()
        .bucket("test-bucket")
        .key("large-file.zip")
        .storage_class(StorageClass::Archive)
        .content_type("application/octet-stream")
        .forbid_overwrite(true)
        .tagging("TagA=A&TagB=B");
}

#[tokio::test]
async fn test_initiate_multipart_upload_builder_with_encryption() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带加密选项的构建器可以创建
    let _builder = client
        .initiate_multipart_upload()
        .bucket("test-bucket")
        .key("large-file.zip")
        .server_side_encryption(ServerSideEncryption::KMS)
        .server_side_encryption_key_id("key-id-123");
}

#[tokio::test]
async fn test_initiate_multipart_upload_builder_with_metadata() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带元数据的构建器可以创建
    let _builder = client
        .initiate_multipart_upload()
        .bucket("test-bucket")
        .key("large-file.zip")
        .metadata("x-oss-meta-author", "test")
        .metadata("x-oss-meta-version", "1.0");
}

#[test]
fn test_storage_class_variants() {
    // 测试 StorageClass 枚举
    assert_eq!(StorageClass::Standard.to_string(), "Standard");
    assert_eq!(StorageClass::IA.to_string(), "IA");
    assert_eq!(StorageClass::Archive.to_string(), "Archive");
    assert_eq!(StorageClass::ColdArchive.to_string(), "ColdArchive");
    assert_eq!(StorageClass::DeepColdArchive.to_string(), "DeepColdArchive");
}

#[test]
fn test_server_side_encryption_variants() {
    // 测试 ServerSideEncryption 枚举
    assert_eq!(ServerSideEncryption::AES256.to_string(), "AES256");
    assert_eq!(ServerSideEncryption::KMS.to_string(), "KMS");
    assert_eq!(ServerSideEncryption::SM4.to_string(), "SM4");
}
