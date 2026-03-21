//! HeadObject 操作实现

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::HeadObjectOutput;

/// HeadObject Fluent Builder
#[derive(Debug)]
pub struct HeadObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: HeadObjectInputBuilder,
}

#[derive(Debug, Default)]
struct HeadObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
}

impl HeadObjectFluentBuilder {
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

    /// 发送请求
    pub async fn send(self) -> Result<HeadObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        let uri = format!("/{}", key);
        let headers = HeaderMap::new();
        let req = self.handle.build_request(
            HttpMethod::Head,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            None,
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
            let storage_class = resp
                .headers()
                .get("x-oss-storage-class")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(HeadObjectOutput {
                content_type,
                content_length,
                etag,
                last_modified,
                storage_class,
                metadata: std::collections::HashMap::new(),
            })
        } else {
            Err(OSSError::Object {
                status_code: status,
                message: "head object failed".to_string(),
                raw_response: serde_json::Value::Null,
            })
        }
    }
}