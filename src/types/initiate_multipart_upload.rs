//! InitiateMultipartUpload 操作类型定义
//!
//! 初始化分片上传任务，获取 UploadId。

use serde::Deserialize;

/// InitiateMultipartUpload 输入
#[derive(Debug, Clone, Default)]
pub struct InitiateMultipartUploadInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// 指定该 Object 被下载时的网页的缓存行为
    pub cache_control: Option<String>,
    /// 指定该 Object 被下载时的名称
    pub content_disposition: Option<String>,
    /// 指定该 Object 被下载时的内容编码格式
    pub content_encoding: Option<String>,
    /// Content-Type
    pub content_type: Option<String>,
    /// 过期时间
    pub expires: Option<String>,
    /// 是否禁止覆盖同名 Object
    pub forbid_overwrite: Option<bool>,
    /// 服务端加密方式
    pub server_side_encryption: Option<String>,
    /// 服务端数据加密算法
    pub server_side_data_encryption: Option<String>,
    /// KMS 托管的用户主密钥
    pub server_side_encryption_key_id: Option<String>,
    /// 存储类型
    pub storage_class: Option<StorageClass>,
    /// Object 标签
    pub tagging: Option<String>,
    /// 用户自定义元数据
    pub metadata: std::collections::HashMap<String, String>,
    /// 编码类型
    pub encoding_type: Option<String>,
}

/// 存储类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StorageClass {
    /// 标准存储
    #[default]
    Standard,
    /// 低频访问
    IA,
    /// 归档存储
    Archive,
    /// 冷归档存储
    ColdArchive,
    /// 深度冷归档存储
    DeepColdArchive,
}

impl std::fmt::Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageClass::Standard => write!(f, "Standard"),
            StorageClass::IA => write!(f, "IA"),
            StorageClass::Archive => write!(f, "Archive"),
            StorageClass::ColdArchive => write!(f, "ColdArchive"),
            StorageClass::DeepColdArchive => write!(f, "DeepColdArchive"),
        }
    }
}

impl From<&str> for StorageClass {
    fn from(s: &str) -> Self {
        match s {
            "Standard" => StorageClass::Standard,
            "IA" => StorageClass::IA,
            "Archive" => StorageClass::Archive,
            "ColdArchive" => StorageClass::ColdArchive,
            "DeepColdArchive" => StorageClass::DeepColdArchive,
            _ => StorageClass::Standard,
        }
    }
}

/// 服务端加密方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerSideEncryption {
    /// AES256 加密
    AES256,
    /// KMS 加密
    KMS,
    /// SM4 加密
    SM4,
}

impl std::fmt::Display for ServerSideEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerSideEncryption::AES256 => write!(f, "AES256"),
            ServerSideEncryption::KMS => write!(f, "KMS"),
            ServerSideEncryption::SM4 => write!(f, "SM4"),
        }
    }
}

impl From<&str> for ServerSideEncryption {
    fn from(s: &str) -> Self {
        match s {
            "AES256" => ServerSideEncryption::AES256,
            "KMS" => ServerSideEncryption::KMS,
            "SM4" => ServerSideEncryption::SM4,
            _ => ServerSideEncryption::AES256,
        }
    }
}

/// InitiateMultipartUpload 输出
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "InitiateMultipartUploadResult")]
pub struct InitiateMultipartUploadOutput {
    /// Bucket 名称
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// Object 名称
    #[serde(rename = "Key")]
    pub key: String,
    /// 分片上传 ID
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    /// 编码类型
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,
}

/// InitiateMultipartUpload 输入构建器
#[derive(Debug, Default)]
pub struct InitiateMultipartUploadInputBuilder {
    inner: InitiateMultipartUploadInput,
}

impl InitiateMultipartUploadInputBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = key.into();
        self
    }

    /// 设置 Cache-Control
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.inner.cache_control = Some(cache_control.into());
        self
    }

    /// 设置 Content-Disposition
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.inner.content_disposition = Some(content_disposition.into());
        self
    }

    /// 设置 Content-Encoding
    pub fn content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.inner.content_encoding = Some(content_encoding.into());
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// 设置 Expires
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.inner.expires = Some(expires.into());
        self
    }

    /// 设置是否禁止覆盖同名 Object
    pub fn forbid_overwrite(mut self, forbid_overwrite: bool) -> Self {
        self.inner.forbid_overwrite = Some(forbid_overwrite);
        self
    }

    /// 设置服务端加密方式
    pub fn server_side_encryption(mut self, encryption: ServerSideEncryption) -> Self {
        self.inner.server_side_encryption = Some(encryption.to_string());
        self
    }

    /// 设置服务端数据加密算法
    pub fn server_side_data_encryption(mut self, encryption: impl Into<String>) -> Self {
        self.inner.server_side_data_encryption = Some(encryption.into());
        self
    }

    /// 设置 KMS 托管的用户主密钥
    pub fn server_side_encryption_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.inner.server_side_encryption_key_id = Some(key_id.into());
        self
    }

    /// 设置存储类型
    pub fn storage_class(mut self, storage_class: StorageClass) -> Self {
        self.inner.storage_class = Some(storage_class);
        self
    }

    /// 设置 Object 标签
    pub fn tagging(mut self, tagging: impl Into<String>) -> Self {
        self.inner.tagging = Some(tagging.into());
        self
    }

    /// 添加用户自定义元数据
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.metadata.insert(key.into(), value.into());
        self
    }

    /// 设置编码类型
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// 构建 Input
    pub fn build(self) -> Result<InitiateMultipartUploadInput, &'static str> {
        if self.inner.bucket.is_empty() {
            return Err("bucket is required");
        }
        if self.inner.key.is_empty() {
            return Err("key is required");
        }
        Ok(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_class_display() {
        assert_eq!(StorageClass::Standard.to_string(), "Standard");
        assert_eq!(StorageClass::IA.to_string(), "IA");
        assert_eq!(StorageClass::Archive.to_string(), "Archive");
        assert_eq!(StorageClass::ColdArchive.to_string(), "ColdArchive");
        assert_eq!(StorageClass::DeepColdArchive.to_string(), "DeepColdArchive");
    }

    #[test]
    fn test_storage_class_from_str() {
        assert_eq!(StorageClass::from("Standard"), StorageClass::Standard);
        assert_eq!(StorageClass::from("IA"), StorageClass::IA);
        assert_eq!(StorageClass::from("Archive"), StorageClass::Archive);
        assert_eq!(StorageClass::from("ColdArchive"), StorageClass::ColdArchive);
        assert_eq!(StorageClass::from("DeepColdArchive"), StorageClass::DeepColdArchive);
        assert_eq!(StorageClass::from("unknown"), StorageClass::Standard);
    }

    #[test]
    fn test_server_side_encryption_display() {
        assert_eq!(ServerSideEncryption::AES256.to_string(), "AES256");
        assert_eq!(ServerSideEncryption::KMS.to_string(), "KMS");
        assert_eq!(ServerSideEncryption::SM4.to_string(), "SM4");
    }

    #[test]
    fn test_input_builder() {
        let input = InitiateMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .key("multipart.data")
            .storage_class(StorageClass::Archive)
            .content_type("application/octet-stream")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "multipart.data");
        assert_eq!(input.storage_class, Some(StorageClass::Archive));
        assert_eq!(input.content_type, Some("application/octet-stream".to_string()));
    }

    #[test]
    fn test_input_builder_missing_bucket() {
        let result = InitiateMultipartUploadInputBuilder::new()
            .key("multipart.data")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "bucket is required");
    }

    #[test]
    fn test_input_builder_missing_key() {
        let result = InitiateMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "key is required");
    }

    #[test]
    fn test_deserialize_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<InitiateMultipartUploadResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Bucket>oss-example</Bucket>
    <Key>multipart.data</Key>
    <UploadId>0004B9894A22E5B1888A1E29F823****</UploadId>
</InitiateMultipartUploadResult>"#;

        let output: InitiateMultipartUploadOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.bucket, "oss-example");
        assert_eq!(output.key, "multipart.data");
        assert_eq!(output.upload_id, "0004B9894A22E5B1888A1E29F823****");
        assert_eq!(output.encoding_type, None);
    }

    #[test]
    fn test_deserialize_output_with_encoding_type() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<InitiateMultipartUploadResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Bucket>oss-example</Bucket>
    <Key>multipart.data</Key>
    <UploadId>0004B9894A22E5B1888A1E29F823****</UploadId>
    <EncodingType>url</EncodingType>
</InitiateMultipartUploadResult>"#;

        let output: InitiateMultipartUploadOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.bucket, "oss-example");
        assert_eq!(output.key, "multipart.data");
        assert_eq!(output.upload_id, "0004B9894A22E5B1888A1E29F823****");
        assert_eq!(output.encoding_type, Some("url".to_string()));
    }

    #[test]
    fn test_input_builder_with_metadata() {
        let input = InitiateMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .key("multipart.data")
            .metadata("x-oss-meta-author", "test")
            .metadata("x-oss-meta-version", "1.0")
            .build()
            .unwrap();

        assert_eq!(input.metadata.get("x-oss-meta-author"), Some(&"test".to_string()));
        assert_eq!(input.metadata.get("x-oss-meta-version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_input_builder_with_encryption() {
        let input = InitiateMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .key("multipart.data")
            .server_side_encryption(ServerSideEncryption::KMS)
            .server_side_encryption_key_id("key-id-123")
            .build()
            .unwrap();

        assert_eq!(input.server_side_encryption, Some("KMS".to_string()));
        assert_eq!(input.server_side_encryption_key_id, Some("key-id-123".to_string()));
    }
}