//! HeadObject Input/Output 类型定义

/// HeadObject 操作输入
#[derive(Debug, Clone)]
pub struct HeadObjectInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
}

impl HeadObjectInput {
    /// 创建新的 HeadObjectInput 构建器
    pub fn builder() -> HeadObjectInputBuilder {
        HeadObjectInputBuilder::default()
    }
}

/// HeadObjectInput 构建器
#[derive(Debug, Default)]
pub struct HeadObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
}

impl HeadObjectInputBuilder {
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

    /// 构建 HeadObjectInput
    pub fn build(self) -> Result<HeadObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(HeadObjectInput { bucket, key })
    }
}

/// HeadObject 操作输出
#[derive(Debug, Default)]
pub struct HeadObjectOutput {
    /// Content-Type
    pub content_type: Option<String>,
    /// Content-Length
    pub content_length: Option<u64>,
    /// ETag
    pub etag: Option<String>,
    /// 最后修改时间
    pub last_modified: Option<String>,
    /// 存储类型
    pub storage_class: Option<String>,
    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}
