//! PutSymlink 操作实现
//!
//! 为 OSS 的目标文件（TargetObject）创建软链接（Symlink）
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
//!     // 创建软链接
//!     let output = client.put_symlink()
//!         .bucket("my-bucket")
//!         .key("link-to-file.txt")
//!         .target("original-file.txt")
//!         .send()
//!         .await?;
//!
//!     println!("Request ID: {:?}", output.request_id);
//!     println!("ETag: {:?}", output.etag);
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::PutSymlinkOutput;

/// PutSymlink Fluent Builder
#[derive(Debug)]
pub struct PutSymlinkFluentBuilder {
    handle: Arc<Handle>,
    inner: PutSymlinkInputBuilder,
}

#[derive(Debug, Default)]
struct PutSymlinkInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    target: Option<String>,
    forbid_overwrite: Option<bool>,
    acl: Option<String>,
    storage_class: Option<String>,
}

impl PutSymlinkFluentBuilder {
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

    /// 设置软链接指向的目标文件
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.inner.target = Some(target.into());
        self
    }

    /// 设置是否禁止覆盖同名 Object
    /// - true: 禁止覆盖
    /// - false: 允许覆盖（默认）
    pub fn forbid_overwrite(mut self, forbid_overwrite: bool) -> Self {
        self.inner.forbid_overwrite = Some(forbid_overwrite);
        self
    }

    /// 设置 Object 的访问权限
    /// 可选值: private, public-read, public-read-write, default
    pub fn acl(mut self, acl: impl Into<String>) -> Self {
        self.inner.acl = Some(acl.into());
        self
    }

    /// 设置 Object 的存储类型
    /// 可选值: Standard, IA, Archive
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.inner.storage_class = Some(storage_class.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `PutSymlinkOutput`，包含请求 ID、ETag 和版本 ID（如果有）
    ///
    /// # 错误
    ///
    /// - 如果 Bucket、Key 或 Target 未设置，返回相应错误
    /// - 如果没有权限，返回 403 错误
    pub async fn send(self) -> Result<PutSymlinkOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let target = self.inner.target.ok_or_else(|| {
            OSSError::InvalidInput(
                "target is required, please call .target() to set it".to_string(),
            )
        })?;

        // 构建请求头
        let mut headers = HeaderMap::new();

        // 设置目标文件
        headers.insert(
            "x-oss-symlink-target",
            HeaderValue::from_str(&target).map_err(OSSError::InvalidHeaderValue)?,
        );

        // 设置是否禁止覆盖
        if let Some(forbid_overwrite) = self.inner.forbid_overwrite {
            if forbid_overwrite {
                headers.insert("x-oss-forbid-overwrite", HeaderValue::from_static("true"));
            }
        }

        // 设置 ACL
        if let Some(ref acl) = self.inner.acl {
            headers.insert(
                "x-oss-object-acl",
                HeaderValue::from_str(acl).map_err(OSSError::InvalidHeaderValue)?,
            );
        }

        // 设置存储类型
        if let Some(ref storage_class) = self.inner.storage_class {
            headers.insert(
                "x-oss-storage-class",
                HeaderValue::from_str(storage_class).map_err(OSSError::InvalidHeaderValue)?,
            );
        }

        let uri = format!("/{}", key);
        let query = "symlink";
        let req = self.handle.build_request(
            HttpMethod::Put,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let request_id = resp
                .headers()
                .get("x-oss-request-id")
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

            Ok(PutSymlinkOutput {
                request_id,
                etag,
                version_id,
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
