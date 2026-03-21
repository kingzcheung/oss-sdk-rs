//! RestoreObject 操作实现
//!
//! 解冻归档、冷归档、深度冷归档类型的 Object。
//!
//! # 示例
//!
//! ```no_run
//! use oss_sdk_rs::{Client, Config, Credentials, Tier};
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
//!     // 解冻归档类型 Object
//!     let output = client.restore_object()
//!         .bucket("my-bucket")
//!         .key("archive-object.txt")
//!         .days(7)
//!         .send()
//!         .await?;
//!     
//!     println!("Status: {:?}", output.status);
//!     
//!     // 解冻冷归档类型 Object（指定优先级）
//!     let output = client.restore_object()
//!         .bucket("my-bucket")
//!         .key("cold-archive-object.txt")
//!         .days(30)
//!         .tier(Tier::Expedited)
//!         .send()
//!         .await?;
//!     
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{to_restore_xml, RestoreObjectOutput, RestoreStatus, Tier};

/// RestoreObject Fluent Builder
#[derive(Debug)]
pub struct RestoreObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: RestoreObjectInputBuilder,
}

#[derive(Debug, Default)]
struct RestoreObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
    days: Option<u32>,
    tier: Option<Tier>,
}

impl RestoreObjectFluentBuilder {
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
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 设置解冻天数
    /// - 归档类型：1~7 天
    /// - 冷归档/深度冷归档：1~365 天
    pub fn days(mut self, days: u32) -> Self {
        self.inner.days = Some(days);
        self
    }

    /// 设置解冻优先级（仅冷归档、深度冷归档有效）
    pub fn tier(mut self, tier: Tier) -> Self {
        self.inner.tier = Some(tier);
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `RestoreObjectOutput`，包含解冻结果
    ///
    /// # 错误
    ///
    /// - 如果 Object 正在解冻中，返回 409 错误
    /// - 如果 Object 不存在，返回 404 错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.restore_object()
    ///     .bucket("my-bucket")
    ///     .key("archive-object.txt")
    ///     .days(7)
    ///     .send()
    ///     .await?;
    ///
    /// println!("Status: {:?}", output.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<RestoreObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let days = self.inner.days.ok_or_else(|| OSSError::InvalidInput("days is required".to_string()))?;

        // 构建 XML 请求体
        let xml_body = to_restore_xml(days, self.inner.tier);

        // 构建查询参数
        let query = if let Some(ref version_id) = self.inner.version_id {
            format!("restore&versionId={}", version_id)
        } else {
            "restore".to_string()
        };

        let uri = format!("/{}", key);

        // 构建请求头
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/xml".parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
        headers.insert(
            "Content-Length",
            xml_body.len().to_string().parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );

        let req = self.handle.build_request(
            HttpMethod::Post,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let resp = req.body(xml_body).send().await?;
        let status = resp.status();
        let resp_headers = resp.headers().clone();

        // 处理不同的状态码
        match status {
            StatusCode::ACCEPTED => {
                // 第一次解冻请求，返回 202
                Ok(RestoreObjectOutput {
                    request_id: resp_headers
                        .get("x-oss-request-id")
                        .and_then(|v| v.to_str().ok())
                        .map(|v| v.to_string()),
                    version_id: resp_headers
                        .get("x-oss-version-id")
                        .and_then(|v| v.to_str().ok())
                        .map(|v| v.to_string()),
                    restore_priority: None,
                    status: RestoreStatus::Accepted,
                })
            }
            StatusCode::OK => {
                // Object 已解冻，再次解冻返回 200
                Ok(RestoreObjectOutput {
                    request_id: resp_headers
                        .get("x-oss-request-id")
                        .and_then(|v| v.to_str().ok())
                        .map(|v| v.to_string()),
                    version_id: resp_headers
                        .get("x-oss-version-id")
                        .and_then(|v| v.to_str().ok())
                        .map(|v| v.to_string()),
                    restore_priority: resp_headers
                        .get("x-oss-object-restore-priority")
                        .and_then(|v| v.to_str().ok())
                        .map(|v| v.to_string()),
                    status: RestoreStatus::Ok,
                })
            }
            StatusCode::CONFLICT => {
                // 正在解冻中
                Err(OSSError::Object {
                    status_code: status,
                    message: "The restore operation is in progress".to_string(),
                    raw_response: serde_json::Value::Null,
                })
            }
            _ => {
                Err(OSSError::Object {
                    status_code: status,
                    message: "Restore object failed".to_string(),
                    raw_response: serde_json::Value::Null,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_bucket_key_days() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.restore_object()
            .bucket("my-bucket")
            .key("my-object.txt")
            .days(7);

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("my-object.txt".to_string()));
        assert_eq!(builder.inner.days, Some(7));
    }

    #[test]
    fn test_builder_with_tier() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.restore_object()
            .bucket("my-bucket")
            .key("my-object.txt")
            .days(30)
            .tier(Tier::Expedited)
            .version_id("version-id");

        assert_eq!(builder.inner.days, Some(30));
        assert_eq!(builder.inner.tier, Some(Tier::Expedited));
        assert_eq!(builder.inner.version_id, Some("version-id".to_string()));
    }
}