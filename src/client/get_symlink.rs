//! GetSymlink 操作实现
//!
//! 获取软链接信息
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
//!         .endpoint("https://oss-cn-hangzhou.aliyuncs.com")
//!         .region("cn-hangzhou")
//!         .build()?;
//!     let client = Client::from_config(config)?;
//!
//!     // 获取软链接
//!     let output = client.get_symlink()
//!         .bucket("my-bucket")
//!         .key("link-to-file.txt")
//!         .send()
//!         .await?;
//!
//!     println!("Target: {:?}", output.target);
//!     println!("ETag: {:?}", output.etag);
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::GetSymlinkOutput;

/// GetSymlink Fluent Builder
#[derive(Debug)]
pub struct GetSymlinkFluentBuilder {
    handle: Arc<Handle>,
    inner: GetSymlinkInputBuilder,
}

#[derive(Debug, Default)]
struct GetSymlinkInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

impl GetSymlinkFluentBuilder {
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

    /// 设置软链接名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 设置版本 ID
    /// 在开启版本控制的 Bucket 中，指定此参数可以获取指定版本的软链接
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `GetSymlinkOutput`，包含软链接指向的目标文件等信息
    ///
    /// # 错误
    ///
    /// - 如果 Bucket 或 Key 未设置，返回相应错误
    /// - 如果软链接不存在，返回 404 错误
    /// - 如果没有权限，返回 403 错误
    pub async fn send(self) -> Result<GetSymlinkOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        // 构建查询参数
        let mut query = String::from("symlink");
        if let Some(ref version_id) = self.inner.version_id {
            query.push_str("&versionId=");
            query.push_str(version_id);
        }

        let headers = HeaderMap::new();
        let uri = format!("/{}", key);
        let req = self.handle.build_request(
            HttpMethod::Get,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let request_id = resp
                .headers()
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let target = resp
                .headers()
                .get("x-oss-symlink-target")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let etag = resp
                .headers()
                .get("ETag")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let version_id = resp
                .headers()
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let last_modified = resp
                .headers()
                .get("Last-Modified")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let delete_marker = resp
                .headers()
                .get("x-oss-delete-marker")
                .and_then(|v| v.to_str().ok())
                .map(|s| s == "true")
                .unwrap_or(false);

            Ok(GetSymlinkOutput {
                target,
                request_id,
                etag,
                version_id,
                last_modified,
                delete_marker,
            })
        } else {
            let text = resp.text().await?;
            Err(OSSError::Object {
                status_code: status,
                message: text,
                raw_response: serde_json::Value::Null,
            })
        }
    }
}
