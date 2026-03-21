//! AppendObject 操作实现
//! 以追加写的方式上传文件

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::AppendObjectOutput;

/// AppendObject Fluent Builder
#[derive(Debug)]
pub struct AppendObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: AppendObjectInputBuilder,
}

#[derive(Debug, Default)]
struct AppendObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    position: Option<u64>,
    body: Option<Vec<u8>>,
    content_type: Option<String>,
    cache_control: Option<String>,
    content_disposition: Option<String>,
    content_md5: Option<String>,
    expires: Option<String>,
    server_side_encryption: Option<String>,
    object_acl: Option<String>,
    storage_class: Option<String>,
    metadata: Option<std::collections::HashMap<String, String>>,
    tagging: Option<String>,
}

impl AppendObjectFluentBuilder {
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

    /// 设置追加位置
    /// 首次追加必须为 0，后续追加为当前 Object 大小
    pub fn position(mut self, position: u64) -> Self {
        self.inner.position = Some(position);
        self
    }

    /// 设置追加内容
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.inner.body = Some(body);
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// 设置缓存控制
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.inner.cache_control = Some(cache_control.into());
        self
    }

    /// 设置内容处置
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.inner.content_disposition = Some(content_disposition.into());
        self
    }

    /// 设置 Content-MD5
    pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
        self.inner.content_md5 = Some(content_md5.into());
        self
    }

    /// 设置过期时间
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.inner.expires = Some(expires.into());
        self
    }

    /// 设置服务端加密方式
    /// 有效值：AES256、KMS、SM4
    pub fn server_side_encryption(mut self, encryption: impl Into<String>) -> Self {
        self.inner.server_side_encryption = Some(encryption.into());
        self
    }

    /// 设置 Object ACL
    /// 有效值：default、private、public-read、public-read-write
    pub fn object_acl(mut self, acl: impl Into<String>) -> Self {
        self.inner.object_acl = Some(acl.into());
        self
    }

    /// 设置存储类型（仅首次追加有效）
    /// 有效值：Standard、IA、Archive
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.inner.storage_class = Some(storage_class.into());
        self
    }

    /// 设置自定义元数据（仅首次追加有效）
    pub fn metadata(mut self, metadata: std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }

    /// 设置 Object 标签（仅首次追加有效）
    pub fn tagging(mut self, tagging: impl Into<String>) -> Self {
        self.inner.tagging = Some(tagging.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `AppendObjectOutput`，包含下一次追加位置等信息
    ///
    /// # 错误
    ///
    /// 如果请求失败，返回 `OSSError`
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use oss_sdk_rs::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // 首次追加
    /// let output = client.append_object()
    ///     .bucket("my-bucket")
    ///     .key("append-file.txt")
    ///     .position(0)
    ///     .body(b"Hello ".to_vec())
    ///     .send()
    ///     .await?;
    ///
    /// // 后续追加
    /// let next_pos = output.next_append_position.unwrap();
    /// let output = client.append_object()
    ///     .bucket("my-bucket")
    ///     .key("append-file.txt")
    ///     .position(next_pos)
    ///     .body(b"World".to_vec())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<AppendObjectOutput, OSSError> {
        let bucket = self
            .inner
            .bucket
            .ok_or_else(|| OSSError::Config("bucket is required for append_object".to_string()))?;
        let key = self
            .inner
            .key
            .ok_or_else(|| OSSError::Config("key is required for append_object".to_string()))?;
        let position = self.inner.position.ok_or_else(|| {
            OSSError::Config("position is required for append_object".to_string())
        })?;
        let body = self
            .inner
            .body
            .ok_or_else(|| OSSError::Config("body is required for append_object".to_string()))?;

        // 构建查询参数
        // POST /ObjectName?append&position=Position
        let query = format!("append&position={}", position);

        // 构建请求头
        let mut headers = HeaderMap::new();

        if let Some(content_type) = &self.inner.content_type {
            headers.insert(
                "Content-Type",
                HeaderValue::from_str(content_type).map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(cache_control) = &self.inner.cache_control {
            headers.insert(
                "Cache-Control",
                HeaderValue::from_str(cache_control)
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(content_disposition) = &self.inner.content_disposition {
            headers.insert(
                "Content-Disposition",
                HeaderValue::from_str(content_disposition)
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(content_md5) = &self.inner.content_md5 {
            headers.insert(
                "Content-MD5",
                HeaderValue::from_str(content_md5).map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(expires) = &self.inner.expires {
            headers.insert(
                "Expires",
                HeaderValue::from_str(expires).map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(encryption) = &self.inner.server_side_encryption {
            headers.insert(
                "x-oss-server-side-encryption",
                HeaderValue::from_str(encryption).map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(acl) = &self.inner.object_acl {
            headers.insert(
                "x-oss-object-acl",
                HeaderValue::from_str(acl).map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(storage_class) = &self.inner.storage_class {
            headers.insert(
                "x-oss-storage-class",
                HeaderValue::from_str(storage_class)
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        if let Some(tagging) = &self.inner.tagging {
            headers.insert(
                "x-oss-tagging",
                HeaderValue::from_str(tagging).map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 添加自定义元数据
        if let Some(metadata) = &self.inner.metadata {
            for (k, v) in metadata {
                let key = format!("x-oss-meta-{}", k);
                headers.insert(
                    reqwest::header::HeaderName::from_bytes(key.as_bytes())
                        .map_err(|e| OSSError::InvalidHeaderName(e))?,
                    HeaderValue::from_str(v).map_err(|e| OSSError::InvalidHeaderValue(e))?,
                );
            }
        }

        // AppendObject 使用 POST 方法
        let req = self.handle.build_request(
            HttpMethod::Post,
            &format!("/{}", key),
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let req = req.body(body);
        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let headers = resp.headers().clone();

            let etag = headers
                .get("ETag")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim_matches('"').to_string());

            let next_append_position = headers
                .get("x-oss-next-append-position")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            let hash_crc64ecma = headers
                .get("x-oss-hash-crc64ecma")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            let request_id = headers
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let version_id = headers
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(AppendObjectOutput {
                etag,
                next_append_position,
                hash_crc64ecma,
                request_id,
                version_id,
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
