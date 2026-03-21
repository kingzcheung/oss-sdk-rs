//! GetObject Input/Output 类型定义

use crate::primitives::ByteStream;
use bytes::Bytes;

/// GetObject 操作输入
#[derive(Debug, Clone)]
pub struct GetObjectInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
    /// 可选：指定返回的字节范围
    pub range: Option<String>,
    /// 可选：指定响应的 content-type
    pub response_content_type: Option<String>,
    /// 可选：指定响应的 content-language
    pub response_content_language: Option<String>,
    /// 可选：指定响应的 expires
    pub response_expires: Option<String>,
    /// 可选：指定响应的 cache-control
    pub response_cache_control: Option<String>,
    /// 可选：指定响应的 content-disposition
    pub response_content_disposition: Option<String>,
    /// 可选：指定响应的 content-encoding
    pub response_content_encoding: Option<String>,
}

impl GetObjectInput {
    /// 创建新的 GetObjectInput
    pub fn builder() -> GetObjectInputBuilder {
        GetObjectInputBuilder::default()
    }
}

/// GetObjectInput 构建器
#[derive(Debug, Default)]
pub struct GetObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    range: Option<String>,
    response_content_type: Option<String>,
    response_content_language: Option<String>,
    response_expires: Option<String>,
    response_cache_control: Option<String>,
    response_content_disposition: Option<String>,
    response_content_encoding: Option<String>,
}

impl GetObjectInputBuilder {
    /// 设置存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 设置对象键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置字节范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 设置响应的 content-type
    pub fn response_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.response_content_type = Some(content_type.into());
        self
    }

    /// 设置响应的 content-disposition
    pub fn response_content_disposition(mut self, disposition: impl Into<String>) -> Self {
        self.response_content_disposition = Some(disposition.into());
        self
    }

    /// 构建 GetObjectInput
    pub fn build(self) -> Result<GetObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(GetObjectInput {
            bucket,
            key,
            range: self.range,
            response_content_type: self.response_content_type,
            response_content_language: self.response_content_language,
            response_expires: self.response_expires,
            response_cache_control: self.response_cache_control,
            response_content_disposition: self.response_content_disposition,
            response_content_encoding: self.response_content_encoding,
        })
    }
}

/// GetObject 操作输出
#[derive(Debug)]
pub struct GetObjectOutput {
    /// 对象内容
    pub body: ByteStream,
    /// Content-Type
    pub content_type: Option<String>,
    /// Content-Length
    pub content_length: Option<u64>,
    /// ETag
    pub etag: Option<String>,
    /// Last-Modified
    pub last_modified: Option<String>,
    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}

impl GetObjectOutput {
    /// 获取 body 作为 Vec<u8>
    pub async fn body_into_vec(self) -> Result<Vec<u8>, std::io::Error> {
        self.body.collect().await
    }

    /// 获取 body 作为 Bytes
    pub fn body_into_bytes(self) -> Result<Bytes, std::io::Error> {
        self.body.into_bytes()
    }
}

impl From<Bytes> for GetObjectOutput {
    fn from(value: Bytes) -> Self {
        Self {
            body: ByteStream::from_bytes(value),
            content_type: None,
            content_length: None,
            etag: None,
            last_modified: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}
