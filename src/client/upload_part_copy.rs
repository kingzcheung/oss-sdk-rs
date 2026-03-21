//! UploadPartCopy 操作实现
//!
//! 从一个已存在的 Object 中拷贝数据来上传一个 Part。
//!
//! # 示例
//!
//! ```no_run
//! use oss_sdk_rs::{Client, Config, Credentials};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new("your-access-key-id", "your-access-key-secret");
//!     let config = Config::builder()
//!         .credentials(credentials)
//!         .region("oss-cn-hangzhou")
//!         .build()?;
//!     let client = Client::from_config(config)?;
//!
//!     // 从源 Object 拷贝数据上传分片
//!     let output = client.upload_part_copy()
//!         .bucket("dest-bucket")
//!         .key("dest-object.zip")
//!         .upload_id("0004B9895DBBB6EC9****")
//!         .part_number(1)
//!         .source_bucket("src-bucket")
//!         .source_key("src-object.zip")
//!         .send()
//!         .await?;
//!
//!     println!("ETag: {:?}", output.copy_part_result.map(|r| r.etag));
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{UploadPartCopyOutput, UploadPartCopyResponse};

/// UploadPartCopy Fluent Builder
#[derive(Debug)]
pub struct UploadPartCopyFluentBuilder {
    handle: Arc<Handle>,
    inner: UploadPartCopyInputBuilder,
}

#[derive(Debug, Default)]
struct UploadPartCopyInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    upload_id: Option<String>,
    part_number: Option<u32>,
    source_bucket: Option<String>,
    source_key: Option<String>,
    source_version_id: Option<String>,
    copy_source_range: Option<String>,
    copy_source_if_match: Option<String>,
    copy_source_if_none_match: Option<String>,
    copy_source_if_unmodified_since: Option<String>,
    copy_source_if_modified_since: Option<String>,
}

impl UploadPartCopyFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置目标 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = Some(bucket.into());
        self
    }

    /// 设置目标 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 设置分片上传 ID
    pub fn upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.inner.upload_id = Some(upload_id.into());
        self
    }

    /// 设置分片号，范围 1~10000
    pub fn part_number(mut self, part_number: u32) -> Self {
        self.inner.part_number = Some(part_number);
        self
    }

    /// 设置源 Bucket 名称
    pub fn source_bucket(mut self, source_bucket: impl Into<String>) -> Self {
        self.inner.source_bucket = Some(source_bucket.into());
        self
    }

    /// 设置源 Object 名称
    pub fn source_key(mut self, source_key: impl Into<String>) -> Self {
        self.inner.source_key = Some(source_key.into());
        self
    }

    /// 设置源 Object 版本 ID
    pub fn source_version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.source_version_id = Some(version_id.into());
        self
    }

    /// 设置拷贝范围，格式：bytes=first-last
    pub fn copy_source_range(mut self, range: impl Into<String>) -> Self {
        self.inner.copy_source_range = Some(range.into());
        self
    }

    /// 设置拷贝范围的起始和结束字节
    /// 自动生成 bytes=first-last 格式
    pub fn copy_source_range_bytes(mut self, first: u64, last: u64) -> Self {
        self.inner.copy_source_range = Some(format!("bytes={}-{}", first, last));
        self
    }

    /// 设置如果源 Object 的 ETag 值和用户提供的 ETag 相等，则执行拷贝操作
    pub fn copy_source_if_match(mut self, etag: impl Into<String>) -> Self {
        self.inner.copy_source_if_match = Some(etag.into());
        self
    }

    /// 设置如果传入的 ETag 值和 Object 的 ETag 不匹配，则正常传输文件
    pub fn copy_source_if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.inner.copy_source_if_none_match = Some(etag.into());
        self
    }

    /// 设置如果传入参数中的时间等于或者晚于文件实际修改时间，则正常传输文件
    pub fn copy_source_if_unmodified_since(mut self, time: impl Into<String>) -> Self {
        self.inner.copy_source_if_unmodified_since = Some(time.into());
        self
    }

    /// 设置如果指定的时间早于实际修改时间，则正常传送文件
    pub fn copy_source_if_modified_since(mut self, time: impl Into<String>) -> Self {
        self.inner.copy_source_if_modified_since = Some(time.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `UploadPartCopyResponse`，包含：
    /// - `copy_part_result`: 拷贝结果，包含 `last_modified` 和 `etag`
    /// - `request_id`: 请求 ID
    /// - `copy_source_version_id`: 源 Object 版本 ID
    ///
    /// # 错误
    ///
    /// - 如果必填参数未设置，返回错误
    /// - 如果 part_number 不在 1~10000 范围内，返回错误
    /// - 如果请求失败，返回 OSS 错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.upload_part_copy()
    ///     .bucket("dest-bucket")
    ///     .key("dest-object.zip")
    ///     .upload_id("upload-id")
    ///     .part_number(1)
    ///     .source_bucket("src-bucket")
    ///     .source_key("src-object.zip")
    ///     .send()
    ///     .await?;
    ///
    /// if let Some(result) = output.copy_part_result {
    ///     println!("ETag: {}", result.etag);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<UploadPartCopyResponse, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let upload_id = self
            .inner
            .upload_id
            .ok_or_else(|| OSSError::InvalidInput("upload_id is required".to_string()))?;
        let part_number = self
            .inner
            .part_number
            .ok_or_else(|| OSSError::InvalidInput("part_number is required".to_string()))?;
        let source_bucket = self
            .inner
            .source_bucket
            .ok_or_else(|| OSSError::InvalidInput("source_bucket is required".to_string()))?;
        let source_key = self
            .inner
            .source_key
            .ok_or_else(|| OSSError::InvalidInput("source_key is required".to_string()))?;

        // 验证 part_number 范围
        if part_number < 1 || part_number > 10000 {
            return Err(OSSError::InvalidInput(
                "part_number must be between 1 and 10000".to_string(),
            ));
        }

        // 构建查询参数
        let query = format!("partNumber={}&uploadId={}", part_number, upload_id);

        let uri = format!("/{}", key);

        // 构建请求头
        let mut headers = HeaderMap::new();

        // 构建拷贝源地址
        let copy_source = if let Some(ref version_id) = self.inner.source_version_id {
            format!("/{}/{}?versionId={}", source_bucket, source_key, version_id)
        } else {
            format!("/{}/{}", source_bucket, source_key)
        };
        headers.insert(
            "x-oss-copy-source",
            copy_source
                .parse()
                .map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );

        // 设置拷贝范围
        if let Some(ref range) = self.inner.copy_source_range {
            headers.insert(
                "x-oss-copy-source-range",
                range.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 设置条件拷贝头
        if let Some(ref etag) = self.inner.copy_source_if_match {
            headers.insert(
                "x-oss-copy-source-if-match",
                etag.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref etag) = self.inner.copy_source_if_none_match {
            headers.insert(
                "x-oss-copy-source-if-none-match",
                etag.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref time) = self.inner.copy_source_if_unmodified_since {
            headers.insert(
                "x-oss-copy-source-if-unmodified-since",
                time.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        if let Some(ref time) = self.inner.copy_source_if_modified_since {
            headers.insert(
                "x-oss-copy-source-if-modified-since",
                time.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        let req = self.handle.build_request(
            HttpMethod::Put,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();
        let resp_headers = resp.headers().clone();

        if status != StatusCode::OK {
            let body = resp.text().await?;
            return Err(OSSError::Object {
                status_code: status,
                message: body,
                raw_response: serde_json::Value::Null,
            });
        }

        let body = resp.text().await?;

        // 解析 XML 响应
        let copy_part_result: Option<UploadPartCopyOutput> = if body.is_empty() {
            None
        } else {
            Some(quick_xml::de::from_str(&body).map_err(|e| OSSError::XmlParse(e.to_string()))?)
        };

        // 解析响应头
        let response = UploadPartCopyResponse {
            copy_part_result,
            request_id: resp_headers
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            copy_source_version_id: resp_headers
                .get("x-oss-copy-source-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_builder_basic() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client
            .upload_part_copy()
            .bucket("dest-bucket")
            .key("dest-object.zip")
            .upload_id("upload-id")
            .part_number(1)
            .source_bucket("src-bucket")
            .source_key("src-object.zip");

        assert_eq!(builder.inner.bucket, Some("dest-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("dest-object.zip".to_string()));
        assert_eq!(builder.inner.upload_id, Some("upload-id".to_string()));
        assert_eq!(builder.inner.part_number, Some(1));
        assert_eq!(builder.inner.source_bucket, Some("src-bucket".to_string()));
        assert_eq!(builder.inner.source_key, Some("src-object.zip".to_string()));
    }

    #[test]
    fn test_builder_with_options() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client
            .upload_part_copy()
            .bucket("dest-bucket")
            .key("dest-object.zip")
            .upload_id("upload-id")
            .part_number(1)
            .source_bucket("src-bucket")
            .source_key("src-object.zip")
            .source_version_id("version-123")
            .copy_source_range_bytes(100, 6291756)
            .copy_source_if_match("etag-123");

        assert_eq!(
            builder.inner.source_version_id,
            Some("version-123".to_string())
        );
        assert_eq!(
            builder.inner.copy_source_range,
            Some("bytes=100-6291756".to_string())
        );
        assert_eq!(
            builder.inner.copy_source_if_match,
            Some("etag-123".to_string())
        );
    }
}
