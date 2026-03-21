//! GetObject 操作实现

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::primitives::ByteStream;
use crate::types::{GetObjectInput, GetObjectOutput};

/// GetObject Fluent Builder
#[derive(Debug)]
pub struct GetObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: GetObjectInputBuilder,
}

#[derive(Debug, Default)]
struct GetObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    range: Option<String>,
    response_content_type: Option<String>,
    response_content_disposition: Option<String>,
}

impl GetObjectFluentBuilder {
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

    /// 设置字节范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.inner.range = Some(range.into());
        self
    }

    /// 设置响应的 Content-Type
    pub fn response_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.response_content_type = Some(content_type.into());
        self
    }

    /// 设置响应的 Content-Disposition
    pub fn response_content_disposition(mut self, disposition: impl Into<String>) -> Self {
        self.inner.response_content_disposition = Some(disposition.into());
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<GetObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        let input = GetObjectInput {
            bucket,
            key,
            range: self.inner.range,
            response_content_type: self.inner.response_content_type,
            response_content_language: None,
            response_expires: None,
            response_cache_control: None,
            response_content_disposition: self.inner.response_content_disposition,
            response_content_encoding: None,
        };

        run_operation(input, self.handle).await
    }
}

async fn run_operation(input: GetObjectInput, handle: Arc<Handle>) -> Result<GetObjectOutput, OSSError> {
    let object_key = &input.key;
    let mut headers = HeaderMap::new();

    // 添加 Range 头
    if let Some(range) = &input.range {
        headers.insert("Range", range.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?);
    }

    // 构建查询参数
    let mut query_parts = Vec::new();
    if let Some(ct) = &input.response_content_type {
        query_parts.push(format!("response-content-type={}", urlencoding::encode(ct)));
    }
    if let Some(cd) = &input.response_content_disposition {
        query_parts.push(format!("response-content-disposition={}", urlencoding::encode(cd)));
    }
    let query = if query_parts.is_empty() {
        None
    } else {
        Some(query_parts.join("&"))
    };

    let uri = format!("/{}", object_key);
    let req = handle.build_request(
        HttpMethod::Get,
        &uri,
        Some(&input.bucket),
        Some(object_key),
        headers,
        query.as_deref(),
    )?;

    let resp = req.send().await?;
    let status = resp.status();

    if status.is_success() {
        let content_type = resp
            .headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let content_length = resp
            .headers()
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());
        let etag = resp
            .headers()
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let last_modified = resp
            .headers()
            .get("Last-Modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let bytes = resp.bytes().await?;

        Ok(GetObjectOutput {
            body: ByteStream::from_bytes(bytes),
            content_type,
            content_length,
            etag,
            last_modified,
            metadata: std::collections::HashMap::new(),
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
