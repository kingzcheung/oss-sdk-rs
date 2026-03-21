//! PutSymlink Input/Output 类型定义
//!
//! 为 OSS 的目标文件（TargetObject）创建软链接（Symlink）

/// PutSymlink 操作输入
#[derive(Debug, Clone)]
pub struct PutSymlinkInput {
    /// 存储桶名称
    pub bucket: String,
    /// 软链接名称
    pub key: String,
    /// 软链接指向的目标文件
    pub target: String,
    /// 是否禁止覆盖同名 Object
    pub forbid_overwrite: Option<bool>,
    /// Object 的访问权限
    pub acl: Option<String>,
    /// Object 的存储类型
    pub storage_class: Option<String>,
}

impl PutSymlinkInput {
    /// 创建新的 PutSymlinkInput 构建器
    pub fn builder() -> PutSymlinkInputBuilder {
        PutSymlinkInputBuilder::default()
    }
}

/// PutSymlinkInput 构建器
#[derive(Debug, Default)]
pub struct PutSymlinkInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    target: Option<String>,
    forbid_overwrite: Option<bool>,
    acl: Option<String>,
    storage_class: Option<String>,
}

impl PutSymlinkInputBuilder {
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

    /// 设置软链接指向的目标文件
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// 设置是否禁止覆盖同名 Object
    /// - true: 禁止覆盖
    /// - false: 允许覆盖（默认）
    pub fn forbid_overwrite(mut self, forbid_overwrite: bool) -> Self {
        self.forbid_overwrite = Some(forbid_overwrite);
        self
    }

    /// 设置 Object 的访问权限
    /// 可选值: private, public-read, public-read-write, default
    pub fn acl(mut self, acl: impl Into<String>) -> Self {
        self.acl = Some(acl.into());
        self
    }

    /// 设置 Object 的存储类型
    /// 可选值: Standard, IA, Archive
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.storage_class = Some(storage_class.into());
        self
    }

    /// 构建 PutSymlinkInput
    pub fn build(self) -> Result<PutSymlinkInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let target = self.target.ok_or("target is required")?;

        Ok(PutSymlinkInput {
            bucket,
            key,
            target,
            forbid_overwrite: self.forbid_overwrite,
            acl: self.acl,
            storage_class: self.storage_class,
        })
    }
}

/// PutSymlink 操作输出
#[derive(Debug, Default)]
pub struct PutSymlinkOutput {
    /// 请求 ID
    pub request_id: Option<String>,
    /// ETag
    pub etag: Option<String>,
    /// 版本 ID（仅在开启版本控制的 Bucket 中返回）
    pub version_id: Option<String>,
}
