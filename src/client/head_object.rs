//! HeadObject 操作实现
//! 获取某个文件（Object）的元数据，不返回文件内容

use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{HeadObjectOutput, ObjectType, RestoreInfo};

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
    version_id: Option<String>,
    if_modified_since: Option<String>,
    if_unmodified_since: Option<String>,
    if_match: Option<String>,
    if_none_match: Option<String>,
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

    /// 设置版本 ID
    /// 在请求参数中指定 versionId，返回指定版本 Object 的元数据
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// 设置 If-Modified-Since 条件
    /// 如果传入参数中的时间早于实际修改时间，则返回 200 OK 和 Object Meta；否则返回 304 Not Modified
    pub fn if_modified_since(mut self, if_modified_since: impl Into<String>) -> Self {
        self.inner.if_modified_since = Some(if_modified_since.into());
        self
    }

    /// 设置 If-Unmodified-Since 条件
    /// 如果传入参数中的时间等于或者晚于文件实际修改时间，则返回 200 OK 和 Object Meta；否则返回 412 Precondition Failed
    pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<String>) -> Self {
        self.inner.if_unmodified_since = Some(if_unmodified_since.into());
        self
    }

    /// 设置 If-Match 条件
    /// 如果传入期望的 ETag 和 Object 的 ETag 匹配，则返回 200 OK 和 Object Meta；否则返回 412 Precondition Failed
    pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
        self.inner.if_match = Some(if_match.into());
        self
    }

    /// 设置 If-None-Match 条件
    /// 如果传入期望的 ETag 值和 Object 的 ETag 不匹配，则返回 200 OK 和 Object Meta；否则返回 304 Not Modified
    pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
        self.inner.if_none_match = Some(if_none_match.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `HeadObjectOutput`，包含 Object 的元数据
    ///
    /// # 错误
    ///
    /// - 如果 Object 不存在，返回 404 错误
    /// - 如果前置条件不满足，返回 412 错误
    /// - 如果 Object 未修改（304），返回 `OSSError::Object` 错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.head_object()
    ///     .bucket("my-bucket")
    ///     .key("my-object.txt")
    ///     .send()
    ///     .await?;
    ///
    /// println!("Content-Type: {:?}", output.content_type);
    /// println!("Content-Length: {:?}", output.content_length);
    /// println!("ETag: {:?}", output.etag);
    /// println!("Last-Modified: {:?}", output.last_modified);
    /// println!("Storage-Class: {:?}", output.storage_class);
    /// println!("Object-Type: {:?}", output.object_type);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<HeadObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;

        let uri = format!("/{}", key);

        // 构建请求头
        let mut headers = HeaderMap::new();

        if let Some(if_modified_since) = &self.inner.if_modified_since {
            headers.insert(
                "If-Modified-Since",
                if_modified_since.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(if_unmodified_since) = &self.inner.if_unmodified_since {
            headers.insert(
                "If-Unmodified-Since",
                if_unmodified_since.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(if_match) = &self.inner.if_match {
            headers.insert(
                "If-Match",
                if_match.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(if_none_match) = &self.inner.if_none_match {
            headers.insert(
                "If-None-Match",
                if_none_match.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 构建查询参数
        let query = self.inner.version_id.as_ref().map(|v| format!("versionId={}", v));

        let req = self.handle.build_request(
            HttpMethod::Head,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            query.as_deref(),
        )?;

        let resp = req.send().await?;
        let status = resp.status();
        let resp_headers = resp.headers().clone();

        // 处理不同的状态码
        match status {
            StatusCode::OK => {
                // 解析响应头
                Ok(parse_head_object_output(&resp_headers))
            }
            StatusCode::NOT_MODIFIED => {
                // 304 Not Modified
                Err(OSSError::Object {
                    status_code: status,
                    message: "Object not modified".to_string(),
                    raw_response: serde_json::Value::Null,
                })
            }
            StatusCode::PRECONDITION_FAILED => {
                // 412 Precondition Failed
                Err(OSSError::Object {
                    status_code: status,
                    message: "Precondition failed".to_string(),
                    raw_response: serde_json::Value::Null,
                })
            }
            StatusCode::NOT_FOUND => {
                // 404 Not Found
                Err(OSSError::Object {
                    status_code: status,
                    message: format!("Object '{}' not found", key),
                    raw_response: serde_json::Value::Null,
                })
            }
            _ => {
                Err(OSSError::Object {
                    status_code: status,
                    message: "Head object failed".to_string(),
                    raw_response: serde_json::Value::Null,
                })
            }
        }
    }
}

/// 解析 HeadObject 响应头
fn parse_head_object_output(headers: &HeaderMap) -> HeadObjectOutput {
    let content_type = headers
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let content_length = headers
        .get("Content-Length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let etag = headers
        .get("ETag")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_matches('"').to_string());

    let last_modified = headers
        .get("Last-Modified")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let storage_class = headers
        .get("x-oss-storage-class")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let object_type = headers
        .get("x-oss-object-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| ObjectType::from(s));

    let version_id = headers
        .get("x-oss-versionId")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let server_side_encryption = headers
        .get("x-oss-server-side-encryption")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let server_side_encryption_key_id = headers
        .get("x-oss-server-side-encryption-key-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let hash_crc64ecma = headers
        .get("x-oss-hash-crc64ecma")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let content_md5 = headers
        .get("Content-Md5")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let next_append_position = headers
        .get("x-oss-next-append-position")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let sealed_time = headers
        .get("x-oss-sealed-time")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let restore = headers
        .get("x-oss-restore")
        .and_then(|v| v.to_str().ok())
        .map(|s| RestoreInfo::from_header(s));

    let tagging_count = headers
        .get("x-oss-tagging-count")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let expiration = headers
        .get("x-oss-expiration")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let transition_time = headers
        .get("x-oss-transition-time")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let request_id = headers
        .get("x-oss-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let cache_control = headers
        .get("Cache-Control")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let content_disposition = headers
        .get("Content-Disposition")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let content_encoding = headers
        .get("Content-Encoding")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let expires = headers
        .get("Expires")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // 解析用户自定义元数据
    let mut metadata = std::collections::HashMap::new();
    for (name, value) in headers.iter() {
        let name_str = name.as_str();
        if name_str.starts_with("x-oss-meta-") {
            if let Ok(v) = value.to_str() {
                let key = name_str.strip_prefix("x-oss-meta-").unwrap_or(name_str);
                metadata.insert(key.to_string(), v.to_string());
            }
        }
    }

    HeadObjectOutput {
        content_type,
        content_length,
        etag,
        last_modified,
        storage_class,
        object_type,
        version_id,
        server_side_encryption,
        server_side_encryption_key_id,
        hash_crc64ecma,
        content_md5,
        next_append_position,
        sealed_time,
        restore,
        tagging_count,
        expiration,
        transition_time,
        request_id,
        metadata,
        cache_control,
        content_disposition,
        content_encoding,
        expires,
    }
}
