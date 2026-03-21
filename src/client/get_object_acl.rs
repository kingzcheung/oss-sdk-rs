//! GetObjectACL 操作实现
//!
//! 获取存储空间（Bucket）下某个文件（Object）的访问权限（ACL）
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
//!     // 获取 Object ACL
//!     let output = client.get_object_acl()
//!         .bucket("my-bucket")
//!         .key("my-object.txt")
//!         .send()
//!         .await?;
//!
//!     println!("Owner ID: {}", output.owner.id);
//!     println!("ACL Grant: {}", output.grant);
//!
//!     Ok(())
//! }
//! ```

use std::str::FromStr;
use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{AccessControlPolicy, GetObjectAclOutput, ObjectAclPermission};

/// GetObjectACL Fluent Builder
#[derive(Debug)]
pub struct GetObjectAclFluentBuilder {
    handle: Arc<Handle>,
    inner: GetObjectAclInputBuilder,
}

#[derive(Debug, Default)]
struct GetObjectAclInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

impl GetObjectAclFluentBuilder {
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
    /// 在开启版本控制的 Bucket 中，指定此参数可以获取指定版本 Object 的 ACL
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `GetObjectAclOutput`，包含 Object 的 ACL 信息
    ///
    /// # 错误
    ///
    /// - 如果 Bucket 或 Key 未设置，返回 `OSSError::BucketNotSet` 或 `OSSError::KeyNotSet`
    /// - 如果 Object 不存在，返回 404 错误
    /// - 如果没有权限，返回 403 错误
    pub async fn send(self) -> Result<GetObjectAclOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        // 构建查询参数
        let mut query = String::from("acl");
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

            let version_id = resp
                .headers()
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let body = resp.text().await?;

            // 解析 XML 响应
            let policy: AccessControlPolicy = AccessControlPolicy::parse(&body)
                .map_err(|e| OSSError::XmlParse(format!("Failed to parse ACL response: {}", e)))?;

            // 解析 ACL 权限
            let grant_str = policy.access_control_list.grant.trim();
            let grant = ObjectAclPermission::from_str(grant_str)
                .map_err(|e| OSSError::XmlParse(format!("Failed to parse ACL grant: {}", e)))?;

            Ok(GetObjectAclOutput {
                owner: policy.owner,
                grant,
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
