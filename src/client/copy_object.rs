//! CopyObject 操作实现

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::CopyObjectOutput;

/// CopyObject Fluent Builder
#[derive(Debug)]
pub struct CopyObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: CopyObjectInputBuilder,
}

#[derive(Debug, Default)]
struct CopyObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    copy_source: Option<String>,
}

impl CopyObjectFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置目标存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = Some(bucket.into());
        self
    }

    /// 设置目标对象键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 设置源对象（格式：bucket/key）
    pub fn copy_source(mut self, copy_source: impl Into<String>) -> Self {
        self.inner.copy_source = Some(copy_source.into());
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<CopyObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let copy_source = self
            .inner
            .copy_source
            .ok_or_else(|| OSSError::Config("copy_source is required".to_string()))?;

        let uri = format!("/{}", key);
        let mut headers = HeaderMap::new();

        // x-oss-copy-source 格式必须以 / 开头，如：/source-bucket/source-key
        let copy_source_value = if copy_source.starts_with('/') {
            copy_source
        } else {
            format!("/{}", copy_source)
        };
        headers.insert(
            "x-oss-copy-source",
            HeaderValue::from_str(&copy_source_value)
                .map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );

        let req = self.handle.build_request(
            HttpMethod::Put,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            None,
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let etag = resp
                .headers()
                .get("ETag")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());
            let request_id = resp
                .headers()
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(CopyObjectOutput {
                etag,
                last_modified: None,
                request_id,
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
