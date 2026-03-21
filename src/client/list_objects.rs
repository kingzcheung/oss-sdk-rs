//! ListObjects 操作实现

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::ListObjectsOutput;

/// ListObjects Fluent Builder
#[derive(Debug)]
pub struct ListObjectsFluentBuilder {
    handle: Arc<Handle>,
    inner: ListObjectsInputBuilder,
}

#[derive(Debug, Default)]
struct ListObjectsInputBuilder {
    bucket: Option<String>,
    prefix: Option<String>,
    delimiter: Option<String>,
    marker: Option<String>,
    max_keys: Option<i32>,
}

impl ListObjectsFluentBuilder {
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

    /// 设置前缀
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.inner.prefix = Some(prefix.into());
        self
    }

    /// 设置分隔符
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.inner.delimiter = Some(delimiter.into());
        self
    }

    /// 设置标记位置
    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.inner.marker = Some(marker.into());
        self
    }

    /// 设置最大返回数量
    pub fn max_keys(mut self, max_keys: i32) -> Self {
        self.inner.max_keys = Some(max_keys);
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<ListObjectsOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;

        // 构建查询参数
        let mut query_parts = Vec::new();
        query_parts.push("list-type=2".to_string());

        if let Some(prefix) = &self.inner.prefix {
            query_parts.push(format!("prefix={}", urlencoding::encode(prefix)));
        }
        if let Some(delimiter) = &self.inner.delimiter {
            query_parts.push(format!("delimiter={}", urlencoding::encode(delimiter)));
        }
        if let Some(marker) = &self.inner.marker {
            query_parts.push(format!(
                "continuation-token={}",
                urlencoding::encode(marker)
            ));
        }
        if let Some(max_keys) = self.inner.max_keys {
            query_parts.push(format!("max-keys={}", max_keys));
        }

        let query = query_parts.join("&");

        let headers = HeaderMap::new();
        let req = self.handle.build_request(
            HttpMethod::Get,
            "/",
            Some(&bucket),
            None,
            headers,
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let text = resp.text().await?;
            let output: ListObjectsOutput =
                quick_xml::de::from_str(&text).map_err(|e| OSSError::XmlParse(e.to_string()))?;
            Ok(output)
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
