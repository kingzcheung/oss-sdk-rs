//! ListParts 操作实现
//!
//! 列举指定 Upload ID 所属的所有已经上传成功 Part

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::ListPartsOutput;

/// ListParts Fluent Builder
#[derive(Debug)]
pub struct ListPartsFluentBuilder {
    handle: Arc<Handle>,
    inner: ListPartsInputBuilder,
}

#[derive(Debug, Default)]
struct ListPartsInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    upload_id: Option<String>,
    max_parts: Option<u32>,
    part_number_marker: Option<u32>,
    encoding_type: Option<String>,
}

impl ListPartsFluentBuilder {
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

    /// 设置返回的最大 Part 数目，默认值 1000，最大值 1000
    pub fn max_parts(mut self, max_parts: u32) -> Self {
        self.inner.max_parts = Some(max_parts);
        self
    }

    /// 设置 List 的起始位置，只有 Part Number 大于该参数的 Part 会被列出
    pub fn part_number_marker(mut self, part_number_marker: u32) -> Self {
        self.inner.part_number_marker = Some(part_number_marker);
        self
    }

    /// 设置编码类型
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<ListPartsOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let upload_id = self.inner.upload_id.ok_or_else(|| {
            OSSError::InvalidInput("upload_id is required".to_string())
        })?;

        // 构建查询参数
        let mut query_parts: Vec<String> = vec![format!("uploadId={}", upload_id)];

        if let Some(max_parts) = self.inner.max_parts {
            query_parts.push(format!("max-parts={}", max_parts));
        }
        if let Some(part_number_marker) = self.inner.part_number_marker {
            query_parts.push(format!("part-number-marker={}", part_number_marker));
        }
        if let Some(ref encoding_type) = self.inner.encoding_type {
            query_parts.push(format!("encoding-type={}", urlencoding::encode(encoding_type)));
        }

        let query = query_parts.join("&");
        let uri = format!("/{}?{}", key, query);
        let headers = HeaderMap::new();

        let req = self.handle.build_request(
            HttpMethod::Get,
            &uri,
            Some(&bucket),
            Some(&key),
            headers,
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let body = resp.bytes().await?;
            let output: ListPartsOutput = quick_xml::de::from_reader(&*body)
                .map_err(|e| OSSError::XmlParse(format!("Failed to parse response: {}", e)))?;
            Ok(output)
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
    async fn test_list_parts_builder() {
        let config = Config::builder()
            .credentials(Credentials::new("test-key", "test-secret"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = Client::from_config(config).unwrap();

        // 测试基本构建器可以创建
        let _builder = client
            .list_parts()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id-123");
    }

    #[tokio::test]
    async fn test_list_parts_builder_with_options() {
        let config = Config::builder()
            .credentials(Credentials::new("test-key", "test-secret"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = Client::from_config(config).unwrap();

        // 测试带选项的构建器
        let _builder = client
            .list_parts()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id-123")
            .max_parts(100)
            .part_number_marker(10)
            .encoding_type("url");
    }
}