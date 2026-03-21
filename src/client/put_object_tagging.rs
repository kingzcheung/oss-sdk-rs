//! PutObjectTagging 操作实现
//!
//! 设置或更新对象（Object）的标签（Tagging）信息
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
//!     // 设置对象标签
//!     let output = client.put_object_tagging()
//!         .bucket("my-bucket")
//!         .key("my-object.txt")
//!         .tag("env", "production")
//!         .tag("team", "backend")
//!         .send()
//!         .await?;
//!
//!     println!("Request ID: {}", output.request_id);
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::PutObjectTaggingOutput;

/// PutObjectTagging Fluent Builder
#[derive(Debug)]
pub struct PutObjectTaggingFluentBuilder {
    handle: Arc<Handle>,
    inner: PutObjectTaggingInputBuilder,
}

#[derive(Debug, Default)]
struct PutObjectTaggingInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    tags: Vec<(String, String)>,
    version_id: Option<String>,
}

impl PutObjectTaggingFluentBuilder {
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

    /// 设置对象名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 添加标签
    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.tags.push((key.into(), value.into()));
        self
    }

    /// 设置版本 ID
    /// 在开启版本控制的 Bucket 中，指定此参数可以设置指定版本 Object 的标签
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `PutObjectTaggingOutput`，包含请求 ID 等信息
    ///
    /// # 错误
    ///
    /// - 如果 Bucket 或 Key 未设置，返回相应错误
    /// - 如果没有标签，返回错误
    /// - 如果没有权限，返回 403 错误
    pub async fn send(self) -> Result<PutObjectTaggingOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        if self.inner.tags.is_empty() {
            return Err(OSSError::InvalidInput(
                "at least one tag is required".to_string(),
            ));
        }

        // 构建 XML 请求体
        let mut xml =
            String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Tagging>\n  <TagSet>\n");
        for (k, v) in &self.inner.tags {
            xml.push_str(&format!(
                "    <Tag>\n      <Key>{}</Key>\n      <Value>{}</Value>\n    </Tag>\n",
                escape_xml(k),
                escape_xml(v)
            ));
        }
        xml.push_str("  </TagSet>\n</Tagging>");

        // 构建查询参数
        let mut query = String::from("tagging");
        if let Some(ref version_id) = self.inner.version_id {
            query.push_str("&versionId=");
            query.push_str(version_id);
        }

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/xml"));

        let uri = format!("/{}", key);
        let req = self.handle.build_request(
            HttpMethod::Put,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let resp = req.body(xml).send().await?;
        let status = resp.status();

        if status.is_success() {
            let request_id = resp
                .headers()
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
                .unwrap_or_default();

            let version_id = resp
                .headers()
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(PutObjectTaggingOutput {
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

/// XML 转义
fn escape_xml(s: &str) -> String {
    s.replace('&', "\u{0026}amp;")
        .replace('<', "\u{0026}lt;")
        .replace('>', "\u{0026}gt;")
        .replace('"', "\u{0026}quot;")
        .replace('\'', "\u{0026}apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a&b"), "a&amp;b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
    }
}
