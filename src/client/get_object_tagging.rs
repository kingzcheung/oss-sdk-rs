//! GetObjectTagging 操作实现
//!
//! 获取对象（Object）的标签（Tagging）信息
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
//!     // 获取对象标签
//!     let output = client.get_object_tagging()
//!         .bucket("my-bucket")
//!         .key("my-object.txt")
//!         .send()
//!         .await?;
//!
//!     println!("Request ID: {}", output.request_id);
//!     for (key, value) in output.tags() {
//!         println!("Tag: {} = {}", key, value);
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{GetObjectTaggingOutput, ObjectTag, Tagging};

/// GetObjectTagging Fluent Builder
#[derive(Debug)]
pub struct GetObjectTaggingFluentBuilder {
    handle: Arc<Handle>,
    inner: GetObjectTaggingInputBuilder,
}

#[derive(Debug, Default)]
struct GetObjectTaggingInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

impl GetObjectTaggingFluentBuilder {
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

    /// 设置版本 ID
    /// 在开启版本控制的 Bucket 中，指定此参数可以获取指定版本 Object 的标签
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `GetObjectTaggingOutput`，包含标签信息
    ///
    /// # 错误
    ///
    /// - 如果 Bucket 或 Key 未设置，返回相应错误
    /// - 如果对象不存在，返回 404 错误
    /// - 如果没有权限，返回 403 错误
    pub async fn send(self) -> Result<GetObjectTaggingOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        // 构建查询参数
        let mut query = String::from("tagging");
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
                .map(|s| s.to_string())
                .unwrap_or_default();

            let version_id = resp
                .headers()
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            // 读取响应体并解析 XML
            let body = resp.text().await?;
            let tagging = parse_tagging_xml(&body)?;

            Ok(GetObjectTaggingOutput {
                request_id,
                version_id,
                tagging,
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

/// 解析标签 XML
fn parse_tagging_xml(xml: &str) -> Result<Tagging, OSSError> {
    let mut tag_set = Vec::new();

    // 简单的 XML 解析 - 查找所有 <Tag> 元素
    let mut pos = 0;
    while let Some(start) = xml[pos..].find("<Tag>") {
        let tag_content_start = pos + start + 5;
        if let Some(end) = xml[tag_content_start..].find("</Tag>") {
            let tag_content = &xml[tag_content_start..tag_content_start + end];

            // 解析 Key
            let key = if let Some(key_start) = tag_content.find("<Key>") {
                if let Some(key_end) = tag_content.find("</Key>") {
                    let key = &tag_content[key_start + 5..key_end];
                    Some(unescape_xml(key))
                } else {
                    None
                }
            } else {
                None
            };

            // 解析 Value
            let value = if let Some(value_start) = tag_content.find("<Value>") {
                if let Some(value_end) = tag_content.find("</Value>") {
                    let value = &tag_content[value_start + 7..value_end];
                    Some(unescape_xml(value))
                } else {
                    None
                }
            } else {
                None
            };

            if let (Some(k), Some(v)) = (key, value) {
                tag_set.push(ObjectTag { key: k, value: v });
            }

            pos = tag_content_start + end + 6;
        } else {
            break;
        }
    }

    Ok(Tagging { tag_set })
}

/// XML 反转义
fn unescape_xml(s: &str) -> String {
    s.replace("\u{0026}amp;", "&")
        .replace("\u{0026}lt;", "<")
        .replace("\u{0026}gt;", ">")
        .replace("\u{0026}quot;", "\"")
        .replace("\u{0026}apos;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tagging_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Tagging>
  <TagSet>
    <Tag>
      <Key>a</Key>
      <Value>1</Value>
    </Tag>
    <Tag>
      <Key>b</Key>
      <Value>2</Value>
    </Tag>
  </TagSet>
</Tagging>"#;

        let tagging = parse_tagging_xml(xml).unwrap();
        assert_eq!(tagging.tag_set.len(), 2);
        assert_eq!(tagging.tag_set[0].key, "a");
        assert_eq!(tagging.tag_set[0].value, "1");
        assert_eq!(tagging.tag_set[1].key, "b");
        assert_eq!(tagging.tag_set[1].value, "2");
    }
}
