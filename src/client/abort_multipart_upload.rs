//! AbortMultipartUpload 操作实现
//!
//! 取消 MultipartUpload 事件并删除对应的 Part 数据

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::AbortMultipartUploadOutput;

/// AbortMultipartUpload Fluent Builder
#[derive(Debug)]
pub struct AbortMultipartUploadFluentBuilder {
    handle: Arc<Handle>,
    inner: AbortMultipartUploadInputBuilder,
}

#[derive(Debug, Default)]
struct AbortMultipartUploadInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    upload_id: Option<String>,
}

impl AbortMultipartUploadFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = Some(bucket.into());
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 设置 uploadId
    pub fn upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.inner.upload_id = Some(upload_id.into());
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<AbortMultipartUploadOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let upload_id = self
            .inner
            .upload_id
            .ok_or_else(|| OSSError::InvalidInput("upload_id is required".to_string()))?;

        let uri = format!("/{}?uploadId={}", key, upload_id);
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
                .map(|s| s.to_string())
                .unwrap_or_default();

            let etag = resp
                .headers()
                .get("ETag")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(AbortMultipartUploadOutput { request_id, etag })
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

#[cfg(test)]
mod tests {
    use crate::{Client, Config, Credentials};

    #[tokio::test]
    async fn test_abort_multipart_upload_builder() {
        let config = Config::builder()
            .credentials(Credentials::new("test-key", "test-secret"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = Client::from_config(config).unwrap();

        // 测试构建器可以创建
        let _builder = client
            .abort_multipart_upload()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id-123");
    }
}
