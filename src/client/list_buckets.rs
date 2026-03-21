//! ListBuckets（GetService）操作实现

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::ListBucketsOutput;

/// ListBuckets Fluent Builder
#[derive(Debug)]
pub struct ListBucketsFluentBuilder {
    handle: Arc<Handle>,
    inner: ListBucketsInputBuilder,
}

#[derive(Debug, Default)]
struct ListBucketsInputBuilder {
    prefix: Option<String>,
    marker: Option<String>,
    max_keys: Option<i32>,
    resource_group_id: Option<String>,
}

impl ListBucketsFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置前缀，限定返回的 Bucket 名称必须以 prefix 作为前缀
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.inner.prefix = Some(prefix.into());
        self
    }

    /// 设置标记位置，设定结果从 marker 之后按字母排序的第一个开始返回
    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.inner.marker = Some(marker.into());
        self
    }

    /// 设置最大返回数量，限定此次返回 Bucket 的最大个数（1~1000）
    pub fn max_keys(mut self, max_keys: i32) -> Self {
        self.inner.max_keys = Some(max_keys);
        self
    }

    /// 设置资源组 ID，指定资源组 ID 则返回属于该资源组的所有 Bucket
    pub fn resource_group_id(mut self, resource_group_id: impl Into<String>) -> Self {
        self.inner.resource_group_id = Some(resource_group_id.into());
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<ListBucketsOutput, OSSError> {
        // 构建查询参数
        let mut query_parts = Vec::new();

        if let Some(prefix) = &self.inner.prefix {
            query_parts.push(format!("prefix={}", urlencoding::encode(prefix)));
        }
        if let Some(marker) = &self.inner.marker {
            query_parts.push(format!("marker={}", urlencoding::encode(marker)));
        }
        if let Some(max_keys) = self.inner.max_keys {
            query_parts.push(format!("max-keys={}", max_keys));
        }

        let query = if query_parts.is_empty() {
            None
        } else {
            Some(query_parts.join("&"))
        };

        // 构建请求头
        let mut headers = HeaderMap::new();
        if let Some(resource_group_id) = &self.inner.resource_group_id {
            headers.insert(
                "x-oss-resource-group-id",
                HeaderValue::from_str(resource_group_id)
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // ListBuckets 请求不需要 bucket 参数，直接请求根路径
        let req = self.handle.build_request(
            HttpMethod::Get,
            "/",
            None, // 不指定 bucket
            None, // 不指定 object key
            headers,
            query.as_deref(),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let text = resp.text().await?;
            let output: ListBucketsOutput =
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
