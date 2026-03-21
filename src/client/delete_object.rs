//! DeleteObject 操作实现

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::DeleteObjectOutput;

/// DeleteObject Fluent Builder
#[derive(Debug)]
pub struct DeleteObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: DeleteObjectInputBuilder,
}

#[derive(Debug, Default)]
struct DeleteObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
}

impl DeleteObjectFluentBuilder {
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
    pub async fn send(self) -> Result<DeleteObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        let uri = format!("/{}", key);
        let headers = HeaderMap::new();
        let req = self.handle.build_request(
            HttpMethod::Delete,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            None,
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let request_id = resp
                .headers()
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(DeleteObjectOutput {
                deleted: true,
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
