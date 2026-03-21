//! CompleteMultipartUpload 操作实现
//!
//! 完成分片上传，将所有已上传的 Part 合并成一个完整的 Object。
//!
//! # 示例
//!
//! ```no_run
//! use oss_sdk_rs::{Client, Config, Credentials, PartItem};
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
//!     // 完成分片上传
//!     let output = client.complete_multipart_upload()
//!         .bucket("my-bucket")
//!         .key("large-file.zip")
//!         .upload_id("0004B9895DBBB6EC9****")
//!         .part(1, "\"etag-1\"")
//!         .part(2, "\"etag-2\"")
//!         .send()
//!         .await?;
//!
//!     if let Some(result) = output.result {
//!         println!("Location: {}", result.location);
//!         println!("ETag: {}", result.etag);
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{
    to_complete_multipart_upload_xml, CompleteMultipartUploadOutput,
    CompleteMultipartUploadResponse, ObjectAcl, PartItem,
};

/// CompleteMultipartUpload Fluent Builder
#[derive(Debug)]
pub struct CompleteMultipartUploadFluentBuilder {
    handle: Arc<Handle>,
    inner: CompleteMultipartUploadInputBuilder,
}

#[derive(Debug, Default)]
struct CompleteMultipartUploadInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    upload_id: Option<String>,
    parts: Vec<PartItem>,
    forbid_overwrite: Option<bool>,
    complete_all: bool,
    object_acl: Option<String>,
    encoding_type: Option<String>,
}

impl CompleteMultipartUploadFluentBuilder {
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

    /// 添加 Part
    pub fn part(mut self, part_number: u32, etag: impl Into<String>) -> Self {
        self.inner.parts.push(PartItem::new(part_number, etag));
        self
    }

    /// 设置 Part 列表
    pub fn parts(mut self, parts: Vec<PartItem>) -> Self {
        self.inner.parts = parts;
        self
    }

    /// 设置是否禁止覆盖同名 Object
    pub fn forbid_overwrite(mut self, forbid_overwrite: bool) -> Self {
        self.inner.forbid_overwrite = Some(forbid_overwrite);
        self
    }

    /// 设置是否自动完成所有 Part
    /// 如果设置为 true，则 OSS 会列举当前 UploadId 已上传的所有 Part
    pub fn complete_all(mut self, complete_all: bool) -> Self {
        self.inner.complete_all = complete_all;
        self
    }

    /// 设置 Object ACL
    pub fn object_acl(mut self, acl: ObjectAcl) -> Self {
        self.inner.object_acl = Some(acl.to_string());
        self
    }

    /// 设置编码类型
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `CompleteMultipartUploadResponse`，包含：
    /// - `result`: 完成结果，包含 `location`、`bucket`、`key`、`etag`
    /// - `request_id`: 请求 ID
    /// - `version_id`: Object 版本 ID
    /// - `server_side_encryption`: 服务端加密算法
    ///
    /// # 错误
    ///
    /// - 如果必填参数未设置，返回错误
    /// - 如果请求失败，返回 OSS 错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.complete_multipart_upload()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .upload_id("upload-id")
    ///     .part(1, "\"etag-1\"")
    ///     .send()
    ///     .await?;
    ///
    /// if let Some(result) = output.result {
    ///     println!("ETag: {}", result.etag);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<CompleteMultipartUploadResponse, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let upload_id = self.inner.upload_id.ok_or_else(|| {
            OSSError::InvalidInput("upload_id is required".to_string())
        })?;

        // 如果不是 complete_all，则需要至少一个 Part
        if !self.inner.complete_all && self.inner.parts.is_empty() {
            return Err(OSSError::InvalidInput(
                "parts is required when complete_all is false".to_string(),
            ));
        }

        // 构建查询参数
        let mut query = format!("uploadId={}", upload_id);
        if let Some(ref encoding_type) = self.inner.encoding_type {
            query.push_str(&format!("&encoding-type={}", encoding_type));
        }

        let uri = format!("/{}", key);

        // 构建请求头
        let mut headers = HeaderMap::new();

        // 设置禁止覆盖
        if let Some(forbid_overwrite) = self.inner.forbid_overwrite {
            headers.insert(
                "x-oss-forbid-overwrite",
                forbid_overwrite.to_string().parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 设置自动完成所有 Part
        if self.inner.complete_all {
            headers.insert(
                "x-oss-complete-all",
                "yes".parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 设置 Object ACL
        if let Some(ref acl) = self.inner.object_acl {
            headers.insert(
                "x-oss-object-acl",
                acl.parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 构建请求体
        let body = if self.inner.complete_all {
            // 如果设置了 complete_all，不允许指定 body
            String::new()
        } else {
            to_complete_multipart_upload_xml(&self.inner.parts)
        };

        headers.insert(
            "Content-Type",
            "application/xml".parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
        headers.insert(
            "Content-Length",
            body.len().to_string().parse().map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );

        let req = self.handle.build_request(
            HttpMethod::Post,
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

        let body = resp.text().await?;

        // 解析 XML 响应
        let result: Option<CompleteMultipartUploadOutput> = if body.is_empty() {
            None
        } else {
            Some(quick_xml::de::from_str(&body).map_err(|e| OSSError::XmlParse(e.to_string()))?)
        };

        // 解析响应头
        let response = CompleteMultipartUploadResponse {
            result,
            request_id: resp_headers
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            version_id: resp_headers
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            server_side_encryption: resp_headers
                .get("x-oss-server-side-encryption")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.complete_multipart_upload()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .part(1, "\"etag-1\"")
            .part(2, "\"etag-2\"");

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("large-file.zip".to_string()));
        assert_eq!(builder.inner.upload_id, Some("upload-id".to_string()));
        assert_eq!(builder.inner.parts.len(), 2);
    }

    #[test]
    fn test_builder_with_options() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.complete_multipart_upload()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .part(1, "\"etag-1\"")
            .forbid_overwrite(true)
            .object_acl(ObjectAcl::Private)
            .encoding_type("url");

        assert_eq!(builder.inner.forbid_overwrite, Some(true));
        assert_eq!(builder.inner.object_acl, Some("private".to_string()));
        assert_eq!(builder.inner.encoding_type, Some("url".to_string()));
    }

    #[test]
    fn test_builder_complete_all() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.complete_multipart_upload()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .complete_all(true);

        assert!(builder.inner.complete_all);
        assert!(builder.inner.parts.is_empty());
    }
}