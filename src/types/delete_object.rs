//! DeleteObject Input/Output 类型定义

/// DeleteObject 操作输入
#[derive(Debug, Clone)]
pub struct DeleteObjectInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
}

impl DeleteObjectInput {
    /// 创建新的 DeleteObjectInput 构建器
    pub fn builder() -> DeleteObjectInputBuilder {
        DeleteObjectInputBuilder::default()
    }
}

/// DeleteObjectInput 构建器
#[derive(Debug, Default)]
pub struct DeleteObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
}

impl DeleteObjectInputBuilder {
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

    /// 构建 DeleteObjectInput
    pub fn build(self) -> Result<DeleteObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(DeleteObjectInput { bucket, key })
    }
}

/// DeleteObject 操作输出
#[derive(Debug, Default)]
pub struct DeleteObjectOutput {
    /// 是否删除成功
    pub deleted: bool,
    /// 请求 ID
    pub request_id: Option<String>,
}
