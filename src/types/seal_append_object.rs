//! SealAppendObject 操作 Input/Output 类型定义

/// SealAppendObject 操作输入
#[derive(Debug, Clone, Default)]
pub struct SealAppendObjectInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// Object 的预期长度
    /// OSS 会检查此长度与 Object 的实际长度是否一致
    /// 如果不一致，请求将失败并返回 PositionNotEqualToLength 错误
    pub position: u64,
}

impl SealAppendObjectInput {
    /// 创建新的 SealAppendObjectInput 构建器
    pub fn builder() -> SealAppendObjectInputBuilder {
        SealAppendObjectInputBuilder::default()
    }
}

/// SealAppendObjectInput 构建器
#[derive(Debug, Default)]
pub struct SealAppendObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    position: Option<u64>,
}

impl SealAppendObjectInputBuilder {
    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置 Object 的预期长度
    pub fn position(mut self, position: u64) -> Self {
        self.position = Some(position);
        self
    }

    /// 构建 SealAppendObjectInput
    pub fn build(self) -> Result<SealAppendObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let position = self.position.ok_or("position is required")?;

        Ok(SealAppendObjectInput {
            bucket,
            key,
            position,
        })
    }
}

/// SealAppendObject 操作输出
#[derive(Debug, Clone, Default)]
pub struct SealAppendObjectOutput {
    /// ETag
    pub etag: Option<String>,
    /// Object 类型
    pub object_type: Option<String>,
    /// 存储类型
    pub storage_class: Option<String>,
    /// 封存时间（首次执行 SealAppendObject 的时间）
    pub sealed_time: Option<String>,
    /// 最后修改时间
    pub last_modified: Option<String>,
    /// 内容长度
    pub content_length: Option<u64>,
    /// 内容类型
    pub content_type: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
}
