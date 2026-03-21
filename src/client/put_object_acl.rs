//! PutObjectACL 操作实现
//!
//! 修改文件（Object）的访问权限（ACL）
//!
//! # 示例
//!
//! ```no_run
//! use oss_sdk_rs::{Client, Config, Credentials};
//! use oss_sdk_rs::types::ObjectAclPermission;
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
//!     // 设置 Object ACL 为公共读
//!     let output = client.put_object_acl()
//!         .bucket("my-bucket")
//!         .key("my-object.txt")
//!         .acl(ObjectAclPermission::PublicRead)
//!         .send()
//!         .await?;
//!
//!     println!("Request ID: {:?}", output.request_id);
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{ObjectAclPermission, PutObjectAclOutput};

/// PutObjectACL Fluent Builder
#[derive(Debug)]
pub struct PutObjectAclFluentBuilder {
    handle: Arc<Handle>,
    inner: PutObjectAclInputBuilder,
}

#[derive(Debug, Default)]
struct PutObjectAclInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    acl: Option<ObjectAclPermission>,
    version_id: Option<String>,
}

impl PutObjectAclFluentBuilder {
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

    /// 设置对象 ACL 权限
    ///
    /// 可选值：
    /// - `ObjectAclPermission::Private` - 私有读写
    /// - `ObjectAclPermission::PublicRead` - 公共读
    /// - `ObjectAclPermission::PublicReadWrite` - 公共读写
    /// - `ObjectAclPermission::Default` - 继承 Bucket ACL
    pub fn acl(mut self, acl: ObjectAclPermission) -> Self {
        self.inner.acl = Some(acl);
        self
    }

    /// 设置版本 ID
    /// 在开启版本控制的 Bucket 中，指定此参数可以设置指定版本 Object 的 ACL
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `PutObjectAclOutput`，包含请求 ID 和版本 ID（如果有）
    ///
    /// # 错误
    ///
    /// - 如果 Bucket 或 Key 未设置，返回 `OSSError::BucketNotSet` 或 `OSSError::KeyNotSet`
    /// - 如果 ACL 未设置，返回 `OSSError::InvalidInput`
    /// - 如果 Object 不存在，返回 404 错误
    /// - 如果没有权限，返回 403 错误
    pub async fn send(self) -> Result<PutObjectAclOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let acl = self.inner.acl.ok_or_else(|| {
            OSSError::InvalidInput("acl is required, please call .acl() to set it".to_string())
        })?;

        // 构建查询参数
        let mut query = String::from("acl");
        if let Some(ref version_id) = self.inner.version_id {
            query.push_str("&versionId=");
            query.push_str(version_id);
        }

        // 构建请求头
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-oss-object-acl",
            HeaderValue::from_static(acl.as_str()),
        );

        let uri = format!("/{}", key);
        let req = self.handle.build_request(
            HttpMethod::Put,
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

            let version_id = resp
                .headers()
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(PutObjectAclOutput {
                request_id,
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