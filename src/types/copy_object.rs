//! CopyObject Input/Output 类型定义

/// CopyObject 操作输入
#[derive(Debug, Clone)]
pub struct CopyObjectInput {
    /// 目标存储桶名称
    pub bucket: String,
    /// 目标对象键
    pub key: String,
    /// 源对象（格式：bucket/key）
    pub copy_source: String,
    /// Content-Type
    pub content_type: Option<String>,
    /// 存储类型
    pub storage_class: Option<String>,
    /// 对象 ACL
    pub acl: Option<String>,
    /// 元数据指令
    pub metadata_directive: Option<String>,
}

impl CopyObjectInput {
    /// 创建新的 CopyObjectInput 构建器
    pub fn builder() -> CopyObjectInputBuilder {
        CopyObjectInputBuilder::default()
    }
}

/// CopyObjectInput 构建器
#[derive(Debug, Default)]
pub struct CopyObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    copy_source: Option<String>,
    content_type: Option<String>,
    storage_class: Option<String>,
    acl: Option<String>,
    metadata_directive: Option<String>,
}

impl CopyObjectInputBuilder {
    /// 设置目标存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 设置目标对象键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置源对象（格式：bucket/key）
    pub fn copy_source(mut self, copy_source: impl Into<String>) -> Self {
        self.copy_source = Some(copy_source.into());
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// 设置存储类型
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.storage_class = Some(storage_class.into());
        self
    }

    /// 设置对象 ACL
    pub fn acl(mut self, acl: impl Into<String>) -> Self {
        self.acl = Some(acl.into());
        self
    }

    /// 设置元数据指令
    pub fn metadata_directive(mut self, directive: impl Into<String>) -> Self {
        self.metadata_directive = Some(directive.into());
        self
    }

    /// 构建 CopyObjectInput
    pub fn build(self) -> Result<CopyObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let copy_source = self.copy_source.ok_or("copy_source is required")?;

        Ok(CopyObjectInput {
            bucket,
            key,
            copy_source,
            content_type: self.content_type,
            storage_class: self.storage_class,
            acl: self.acl,
            metadata_directive: self.metadata_directive,
        })
    }
}

/// CopyObject 操作输出
#[derive(Debug, Default)]
pub struct CopyObjectOutput {
    /// ETag
    pub etag: Option<String>,
    /// 最后修改时间
    pub last_modified: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
}
