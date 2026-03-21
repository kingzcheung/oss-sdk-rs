//! UploadPart 操作实现
//!
//! 根据指定的 Object 名和 uploadId 来分片上传数据。
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
//!     // 上传分片
//!     let output = client.upload_part()
//!         .bucket("my-bucket")
//!         .key("large-file.zip")
//!         .upload_id("0004B9895DBBB6EC9****")
//!         .part_number(1)
//!         .body(vec![1, 2, 3, 4, 5])
//!         .send()
//!         .await?;
//!
//!     println!("ETag: {:?}", output.etag);
//!     println!("Content-MD5: {:?}", output.content_md5);
//!     println!("CRC64: {:?}", output.crc64_ecma);
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::UploadPartOutput;

/// UploadPart Fluent Builder
#[derive(Debug)]
pub struct UploadPartFluentBuilder {
    handle: Arc<Handle>,
    inner: UploadPartInputBuilder,
}

#[derive(Debug, Default)]
struct UploadPartInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    upload_id: Option<String>,
    part_number: Option<u32>,
    body: Option<Vec<u8>>,
}

impl UploadPartFluentBuilder {
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

    /// 设置分片数据
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.inner.body = Some(body.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `UploadPartOutput`，包含：
    /// - `etag`: 分片的 ETag（MD5 值）
    /// - `content_md5`: 分片的 MD5 值
    /// - `crc64_ecma`: 分片的 CRC64 值
    /// - `request_id`: 请求 ID
    /// - `server_side_encryption`: 服务端加密算法
    ///
    /// # 错误
    ///
    /// - 如果 Bucket、Key、UploadId、PartNumber 或 Body 未设置，返回错误
    /// - 如果 part_number 不在 1~10000 范围内，返回错误
    /// - 如果请求失败，返回 OSS 错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.upload_part()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .upload_id("upload-id")
    ///     .part_number(1)
    ///     .body(vec![1, 2, 3, 4, 5])
    ///     .send()
    ///     .await?;
    ///
    /// println!("ETag: {:?}", output.etag);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<UploadPartOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let upload_id = self.inner.upload_id.ok_or_else(|| {
            OSSError::InvalidInput("upload_id is required".to_string())
        })?;
        let part_number = self.inner.part_number.ok_or_else(|| {
            OSSError::InvalidInput("part_number is required".to_string())
        })?;
        let body = self.inner.body.ok_or_else(|| {
            OSSError::InvalidInput("body is required".to_string())
        })?;

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
        headers.insert(
            "Content-Length",
            body.len().to_string().parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );

        let req = self.handle.build_request(
            HttpMethod::Put,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let resp = req.body(body).send().await?;
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

        // 解析响应头
        let output = UploadPartOutput {
            etag: resp_headers
                .get("ETag")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            content_md5: resp_headers
                .get("Content-MD5")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            crc64_ecma: resp_headers
                .get("x-oss-hash-crc64ecma")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            request_id: resp_headers
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            server_side_encryption: resp_headers
                .get("x-oss-server-side-encryption")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
        };

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_bucket_key() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.upload_part()
            .bucket("my-bucket")
            .key("multipart.data")
            .upload_id("upload-id")
            .part_number(1)
            .body(vec![1, 2, 3]);

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("multipart.data".to_string()));
        assert_eq!(builder.inner.upload_id, Some("upload-id".to_string()));
        assert_eq!(builder.inner.part_number, Some(1));
        assert_eq!(builder.inner.body, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_builder_with_large_part_number() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.upload_part()
            .bucket("my-bucket")
            .key("multipart.data")
            .upload_id("upload-id")
            .part_number(10000)
            .body(vec![1, 2, 3]);

        assert_eq!(builder.inner.part_number, Some(10000));
    }
}