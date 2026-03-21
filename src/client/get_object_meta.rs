//! GetObjectMeta 操作实现
//!
//! 获取文件的元数据信息，包括 ETag、Size、LastModified 信息，不返回文件内容。
//!
//! # 示例
//!
//! ```no_run
//! use oss_sdk_rs::{Client, Config, Credentials};
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
//!     let output = client.get_object_meta()
//!         .bucket("my-bucket")
//!         .key("my-object.txt")
//!         .send()
//!         .await?;
//!     
//!     println!("ETag: {:?}", output.etag);
//!     println!("Size: {:?}", output.content_length);
//!     println!("Last-Modified: {:?}", output.last_modified);
//!     
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::GetObjectMetaOutput;

/// GetObjectMeta Fluent Builder
#[derive(Debug)]
pub struct GetObjectMetaFluentBuilder {
    handle: Arc<Handle>,
    inner: GetObjectMetaInputBuilder,
}

#[derive(Debug, Default)]
struct GetObjectMetaInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

impl GetObjectMetaFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = Some(bucket.into());
        self
    }

    /// 设置对象键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 设置版本 ID
    /// 在请求参数中指定 versionId，返回指定版本 Object 的元数据
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `GetObjectMetaOutput`，包含 Object 的元数据（ETag、Size、LastModified）
    ///
    /// # 错误
    ///
    /// - 如果 Object 不存在，返回 404 错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.get_object_meta()
    ///     .bucket("my-bucket")
    ///     .key("my-object.txt")
    ///     .send()
    ///     .await?;
    ///
    /// println!("ETag: {:?}", output.etag);
    /// println!("Size: {:?}", output.content_length);
    /// println!("Last-Modified: {:?}", output.last_modified);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<GetObjectMetaOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        let uri = format!("/{}", key);

        // 构建查询参数
        let query = if let Some(ref version_id) = self.inner.version_id {
            format!("objectMeta&versionId={}", version_id)
        } else {
            "objectMeta".to_string()
        };

        let req = self.handle.build_request(
            HttpMethod::Head,
            &uri,
            Some(&bucket),
            Some(&key),
            HeaderMap::new(),
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();
        let resp_headers = resp.headers().clone();

        // 处理不同的状态码
        match status {
            StatusCode::OK => {
                // 解析响应头
                Ok(parse_get_object_meta_output(&resp_headers))
            }
            StatusCode::NOT_FOUND => {
                // 404 Not Found
                Err(OSSError::Object {
                    status_code: status,
                    message: format!("Object '{}' not found", key),
                    raw_response: serde_json::Value::Null,
                })
            }
            _ => Err(OSSError::Object {
                status_code: status,
                message: "Get object meta failed".to_string(),
                raw_response: serde_json::Value::Null,
            }),
        }
    }
}

/// 解析 GetObjectMeta 响应头
fn parse_get_object_meta_output(headers: &HeaderMap) -> GetObjectMetaOutput {
    let content_length = headers
        .get("Content-Length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let etag = headers
        .get("ETag")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_matches('"').to_string());

    let last_modified = headers
        .get("Last-Modified")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let transition_time = headers
        .get("x-oss-transition-time")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let last_access_time = headers
        .get("x-oss-last-access-time")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let version_id = headers
        .get("x-oss-version-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let sealed_time = headers
        .get("x-oss-sealed-time")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let request_id = headers
        .get("x-oss-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    GetObjectMetaOutput {
        content_length,
        etag,
        last_modified,
        transition_time,
        last_access_time,
        version_id,
        sealed_time,
        request_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_bucket_key() {
        let credentials = crate::credentials::Credentials::new("test", "test");
        let config = crate::config::Config::builder()
            .credentials(credentials)
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client
            .get_object_meta()
            .bucket("my-bucket")
            .key("my-object.txt");

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("my-object.txt".to_string()));
        assert_eq!(builder.inner.version_id, None);
    }

    #[test]
    fn test_builder_with_version() {
        let credentials = crate::credentials::Credentials::new("test", "test");
        let config = crate::config::Config::builder()
            .credentials(credentials)
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client
            .get_object_meta()
            .bucket("my-bucket")
            .key("my-object.txt")
            .version_id("123456");

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("my-object.txt".to_string()));
        assert_eq!(builder.inner.version_id, Some("123456".to_string()));
    }
}
