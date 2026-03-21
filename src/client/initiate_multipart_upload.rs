//! InitiateMultipartUpload 操作实现
//!
//! 初始化分片上传任务，获取 UploadId。
//!
//! # 示例
//!
//! ```no_run
//! use oss_sdk_rs::{Client, Config, Credentials, StorageClass};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new("your-access-key-id", "your-access-key-secret");
//!     let config = Config::builder()
//!         .credentials(credentials)
//!         .region("oss-cn-hangzhou")
//!         .build()?;
//!     let client = Client::from_config(config)?;
//!
//!     // 初始化分片上传
//!     let output = client.initiate_multipart_upload()
//!         .bucket("my-bucket")
//!         .key("large-file.zip")
//!         .storage_class(StorageClass::Standard)
//!         .send()
//!         .await?;
//!
//!     println!("Upload ID: {}", output.upload_id);
//!     println!("Bucket: {}", output.bucket);
//!     println!("Key: {}", output.key);
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{InitiateMultipartUploadOutput, StorageClass};

/// InitiateMultipartUpload Fluent Builder
#[derive(Debug)]
pub struct InitiateMultipartUploadFluentBuilder {
    handle: Arc<Handle>,
    inner: InitiateMultipartUploadInputBuilder,
}

#[derive(Debug, Default)]
struct InitiateMultipartUploadInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    cache_control: Option<String>,
    content_disposition: Option<String>,
    content_encoding: Option<String>,
    content_type: Option<String>,
    expires: Option<String>,
    forbid_overwrite: Option<bool>,
    server_side_encryption: Option<String>,
    server_side_data_encryption: Option<String>,
    server_side_encryption_key_id: Option<String>,
    storage_class: Option<StorageClass>,
    tagging: Option<String>,
    metadata: std::collections::HashMap<String, String>,
    encoding_type: Option<String>,
}

impl InitiateMultipartUploadFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = Some(bucket.into());
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
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
    pub fn server_side_encryption(
        mut self,
        encryption: crate::types::ServerSideEncryption,
    ) -> Self {
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

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `InitiateMultipartUploadOutput`，包含：
    /// - `bucket`: Bucket 名称
    /// - `key`: Object 名称
    /// - `upload_id`: 分片上传 ID
    /// - `encoding_type`: 编码类型
    ///
    /// # 错误
    ///
    /// - 如果 Bucket 或 Key 未设置，返回错误
    /// - 如果请求失败，返回 OSS 错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.initiate_multipart_upload()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .send()
    ///     .await?;
    ///
    /// println!("Upload ID: {}", output.upload_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<InitiateMultipartUploadOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        // 构建查询参数
        let mut query = "uploads".to_string();
        if let Some(ref encoding_type) = self.inner.encoding_type {
            query.push_str(&format!("&encoding-type={}", encoding_type));
        }

        let uri = format!("/{}", key);

        // 构建请求头
        let mut headers = HeaderMap::new();

        // 设置标准 HTTP 头
        if let Some(ref cache_control) = self.inner.cache_control {
            headers.insert(
                "Cache-Control",
                cache_control
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref content_disposition) = self.inner.content_disposition {
            headers.insert(
                "Content-Disposition",
                content_disposition
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref content_encoding) = self.inner.content_encoding {
            headers.insert(
                "Content-Encoding",
                content_encoding
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref content_type) = self.inner.content_type {
            headers.insert(
                "Content-Type",
                content_type
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref expires) = self.inner.expires {
            headers.insert(
                "Expires",
                expires
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 设置 OSS 特定头
        if let Some(forbid_overwrite) = self.inner.forbid_overwrite {
            headers.insert(
                "x-oss-forbid-overwrite",
                forbid_overwrite
                    .to_string()
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref encryption) = self.inner.server_side_encryption {
            headers.insert(
                "x-oss-server-side-encryption",
                encryption
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref data_encryption) = self.inner.server_side_data_encryption {
            headers.insert(
                "x-oss-server-side-data-encryption",
                data_encryption
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref key_id) = self.inner.server_side_encryption_key_id {
            headers.insert(
                "x-oss-server-side-encryption-key-id",
                key_id
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref storage_class) = self.inner.storage_class {
            headers.insert(
                "x-oss-storage-class",
                storage_class
                    .to_string()
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref tagging) = self.inner.tagging {
            headers.insert(
                "x-oss-tagging",
                tagging
                    .parse()
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 设置用户自定义元数据
        for (k, v) in &self.inner.metadata {
            let header_name: reqwest::header::HeaderName = k
                .parse()
                .map_err(|_| OSSError::InvalidInput(format!("Invalid header name: {}", k)))?;
            headers.insert(
                header_name,
                v.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        let req = self.handle.build_request(
            HttpMethod::Post,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status != StatusCode::OK {
            let body = resp.text().await?;
            return Err(OSSError::Object {
                status_code: status,
                message: body,
                raw_response: serde_json::Value::Null,
            });
        }

        let body = resp.text().await?;
        let output: InitiateMultipartUploadOutput =
            quick_xml::de::from_str(&body).map_err(|e| OSSError::XmlParse(e.to_string()))?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_bucket_key() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client
            .initiate_multipart_upload()
            .bucket("my-bucket")
            .key("multipart.data");

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("multipart.data".to_string()));
    }

    #[test]
    fn test_builder_with_options() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client
            .initiate_multipart_upload()
            .bucket("my-bucket")
            .key("multipart.data")
            .storage_class(StorageClass::Archive)
            .content_type("application/octet-stream")
            .forbid_overwrite(true)
            .tagging("TagA=A&TagB=B");

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("multipart.data".to_string()));
        assert_eq!(builder.inner.storage_class, Some(StorageClass::Archive));
        assert_eq!(
            builder.inner.content_type,
            Some("application/octet-stream".to_string())
        );
        assert_eq!(builder.inner.forbid_overwrite, Some(true));
        assert_eq!(builder.inner.tagging, Some("TagA=A&TagB=B".to_string()));
    }

    #[test]
    fn test_builder_with_metadata() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client
            .initiate_multipart_upload()
            .bucket("my-bucket")
            .key("multipart.data")
            .metadata("x-oss-meta-author", "test")
            .metadata("x-oss-meta-version", "1.0");

        assert_eq!(
            builder.inner.metadata.get("x-oss-meta-author"),
            Some(&"test".to_string())
        );
        assert_eq!(
            builder.inner.metadata.get("x-oss-meta-version"),
            Some(&"1.0".to_string())
        );
    }
}
