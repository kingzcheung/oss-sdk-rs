//! PutObject 操作实现

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_DISPOSITION, CONTENT_ENCODING};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{PutObjectInput, PutObjectOutput, ContentDisposition};

/// PutObject Fluent Builder
#[derive(Debug)]
pub struct PutObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: PutObjectInputBuilder,
}

#[derive(Debug, Default)]
struct PutObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    body: Option<Vec<u8>>,
    content_type: Option<String>,
    content_disposition: Option<String>,
    content_encoding: Option<String>,
    storage_class: Option<String>,
    acl: Option<String>,
    forbid_overwrite: bool,
}

impl PutObjectFluentBuilder {
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

    /// 设置对象内容
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.inner.body = Some(body.into());
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// 设置 Content-Disposition
    pub fn content_disposition(mut self, disposition: impl Into<String>) -> Self {
        self.inner.content_disposition = Some(disposition.into());
        self
    }

    /// 设置 Content-Encoding
    pub fn content_encoding(mut self, encoding: impl Into<String>) -> Self {
        self.inner.content_encoding = Some(encoding.into());
        self
    }

    /// 设置存储类型
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.inner.storage_class = Some(storage_class.into());
        self
    }

    /// 设置对象 ACL
    pub fn acl(mut self, acl: impl Into<String>) -> Self {
        self.inner.acl = Some(acl.into());
        self
    }

    /// 设置是否禁止覆盖
    pub fn forbid_overwrite(mut self, forbid: bool) -> Self {
        self.inner.forbid_overwrite = forbid;
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<PutObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let body = self.inner.body.unwrap_or_default();

        let input = PutObjectInput {
            bucket,
            key,
            body: crate::primitives::ByteStream::from_vec(body),
            content_type: self.inner.content_type,
            content_disposition: self.inner.content_disposition.map(|s| {
                ContentDisposition::AttachmentWithFileName(s)
            }),
            content_encoding: None,
            storage_class: None,
            acl: None,
            forbid_overwrite: self.inner.forbid_overwrite,
            metadata: std::collections::HashMap::new(),
        };

        run_operation(input, self.handle).await
    }
}

async fn run_operation(input: PutObjectInput, handle: Arc<Handle>) -> Result<PutObjectOutput, OSSError> {
    let object_key = &input.key;
    let mut headers = HeaderMap::new();

    // 设置 ACL
    if let Some(acl) = &input.acl {
        headers.insert(
            "x-oss-object-acl",
            HeaderValue::from_str(&acl.to_string()).map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
    }

    // 设置存储类型
    if let Some(storage_class) = &input.storage_class {
        headers.insert(
            "x-oss-storage-class",
            HeaderValue::from_str(&storage_class.to_string()).map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
    }

    // 设置 Content-Disposition
    if let Some(content_disposition) = &input.content_disposition {
        headers.insert(
            CONTENT_DISPOSITION,
            HeaderValue::from_str(&content_disposition.to_string()).map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
    }

    // 设置 Content-Encoding
    if let Some(content_encoding) = &input.content_encoding {
        headers.insert(
            CONTENT_ENCODING,
            HeaderValue::from_str(&content_encoding.to_string()).map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
    }

    // 设置禁止覆盖
    if input.forbid_overwrite {
        headers.insert(
            "x-oss-forbid-overwrite",
            HeaderValue::from_static("true"),
        );
    }

    // 设置 Content-Type
    if let Some(content_type) = &input.content_type {
        headers.insert(
            "Content-Type",
            HeaderValue::from_str(content_type).map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
    }

    let uri = format!("/{}", object_key);
    let req = handle.build_request(
        HttpMethod::Put,
        &uri,
        Some(&input.bucket),
        Some(object_key),
        headers,
        None,
    )?;

    let body = input.body.collect().await.map_err(|e| OSSError::Io(e))?;
    let resp = req.body(body).send().await?;
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

        Ok(PutObjectOutput { etag, request_id })
    } else {
        let text = resp.text().await?;
        Err(OSSError::Object {
            status_code: status,
            message: text,
            raw_response: serde_json::Value::Null,
        })
    }
}