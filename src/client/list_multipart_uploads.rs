//! ListMultipartUploads 操作实现
//!
//! 列举所有执行中的 Multipart Upload 事件

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::ListMultipartUploadsOutput;

/// ListMultipartUploads Fluent Builder
#[derive(Debug)]
pub struct ListMultipartUploadsFluentBuilder {
    handle: Arc<Handle>,
    inner: ListMultipartUploadsInputBuilder,
}

#[derive(Debug, Default)]
struct ListMultipartUploadsInputBuilder {
    bucket: Option<String>,
    delimiter: Option<String>,
    max_uploads: Option<u32>,
    key_marker: Option<String>,
    prefix: Option<String>,
    upload_id_marker: Option<String>,
    encoding_type: Option<String>,
}

impl ListMultipartUploadsFluentBuilder {
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

    /// 设置分隔符，用于对 Object 名称进行分组
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.inner.delimiter = Some(delimiter.into());
        self
    }

    /// 设置返回的最大 Upload 个数，默认值为 1000，最大值为 1000
    pub fn max_uploads(mut self, max_uploads: u32) -> Self {
        self.inner.max_uploads = Some(max_uploads);
        self
    }

    /// 设置列表的起始 Object 位置
    pub fn key_marker(mut self, key_marker: impl Into<String>) -> Self {
        self.inner.key_marker = Some(key_marker.into());
        self
    }

    /// 设置前缀，限定返回的 Object Key 必须以 prefix 作为前缀
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.inner.prefix = Some(prefix.into());
        self
    }

    /// 设置列表的起始 UploadId 位置
    pub fn upload_id_marker(mut self, upload_id_marker: impl Into<String>) -> Self {
        self.inner.upload_id_marker = Some(upload_id_marker.into());
        self
    }

    /// 设置编码类型
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<ListMultipartUploadsOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;

        // 构建查询参数
        let mut query_parts: Vec<String> = vec!["uploads".to_string()];

        if let Some(ref delimiter) = self.inner.delimiter {
            query_parts.push(format!("delimiter={}", urlencoding::encode(delimiter)));
        }
        if let Some(max_uploads) = self.inner.max_uploads {
            query_parts.push(format!("max-uploads={}", max_uploads));
        }
        if let Some(ref key_marker) = self.inner.key_marker {
            query_parts.push(format!("key-marker={}", urlencoding::encode(key_marker)));
        }
        if let Some(ref prefix) = self.inner.prefix {
            query_parts.push(format!("prefix={}", urlencoding::encode(prefix)));
        }
        if let Some(ref upload_id_marker) = self.inner.upload_id_marker {
            query_parts.push(format!("upload-id-marker={}", urlencoding::encode(upload_id_marker)));
        }
        if let Some(ref encoding_type) = self.inner.encoding_type {
            query_parts.push(format!("encoding-type={}", urlencoding::encode(encoding_type)));
        }

        let query = query_parts.join("&");
        let uri = format!("/?{}", query);
        let headers = HeaderMap::new();

        let req = self.handle.build_request(
            HttpMethod::Get,
            &uri,
            Some(&bucket),
            None,
            headers,
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let body = resp.bytes().await?;
            let output: ListMultipartUploadsOutput = quick_xml::de::from_reader(&*body)
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
    async fn test_list_multipart_uploads_builder() {
        let config = Config::builder()
            .credentials(Credentials::new("test-key", "test-secret"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = Client::from_config(config).unwrap();

        // 测试基本构建器可以创建
        let _builder = client.list_multipart_uploads().bucket("my-bucket");
    }

    #[tokio::test]
    async fn test_list_multipart_uploads_builder_with_options() {
        let config = Config::builder()
            .credentials(Credentials::new("test-key", "test-secret"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = Client::from_config(config).unwrap();

        // 测试带选项的构建器
        let _builder = client
            .list_multipart_uploads()
            .bucket("my-bucket")
            .delimiter("/")
            .max_uploads(100)
            .prefix("photos/")
            .encoding_type("url");
    }
}