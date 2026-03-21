//! GetSymlink Input/Output 类型定义
//!
//! 获取软链接信息

/// GetSymlink 操作输入
#[derive(Debug, Clone)]
pub struct GetSymlinkInput {
    /// 存储桶名称
    pub bucket: String,
    /// 软链接名称
    pub key: String,
    /// 版本 ID（可选）
    pub version_id: Option<String>,
}

impl GetSymlinkInput {
    /// 创建新的 GetSymlinkInput 构建器
    pub fn builder() -> GetSymlinkInputBuilder {
        GetSymlinkInputBuilder::default()
    }
}

/// GetSymlinkInput 构建器
#[derive(Debug, Default)]
pub struct GetSymlinkInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

impl GetSymlinkInputBuilder {
    /// 设置存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 设置软链接名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置版本 ID
    /// 在开启版本控制的 Bucket 中，指定此参数可以获取指定版本的软链接
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 构建 GetSymlinkInput
    pub fn build(self) -> Result<GetSymlinkInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(GetSymlinkInput {
            bucket,
            key,
            version_id: self.version_id,
        })
    }
}

/// GetSymlink 操作输出
#[derive(Debug, Default)]
pub struct GetSymlinkOutput {
    /// 软链接指向的目标文件
    pub target: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// ETag
    pub etag: Option<String>,
    /// 版本 ID（仅在开启版本控制的 Bucket 中返回）
    pub version_id: Option<String>,
    /// 最后修改时间
    pub last_modified: Option<String>,
    /// 是否为删除标记
    pub delete_marker: bool,
}
