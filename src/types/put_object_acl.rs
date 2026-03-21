//! PutObjectACL Input/Output 类型定义
//!
//! 设置 Object 的访问权限（ACL）

/// Object ACL 权限类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectAcl {
    /// 私有读写
    /// 只有 Object 的 Owner 拥有该 Object 的读写权限，其他用户没有权限操作该 Object
    Private,
    /// 公共读
    /// Object Owner 拥有该 Object 的读写权限。非 Object Owner 只有该 Object 的读权限
    PublicRead,
    /// 公共读写
    /// 所有用户拥有对该 Object 的读写权限
    PublicReadWrite,
    /// 默认
    /// Object 遵循其所在 Bucket 的读写权限
    Default,
}

impl ObjectAcl {
    /// 转换为 OSS API 使用的字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectAcl::Private => "private",
            ObjectAcl::PublicRead => "public-read",
            ObjectAcl::PublicReadWrite => "public-read-write",
            ObjectAcl::Default => "default",
        }
    }
}

impl std::fmt::Display for ObjectAcl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ObjectAcl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "private" => Ok(ObjectAcl::Private),
            "public-read" => Ok(ObjectAcl::PublicRead),
            "public-read-write" => Ok(ObjectAcl::PublicReadWrite),
            "default" => Ok(ObjectAcl::Default),
            _ => Err(format!("Invalid Object ACL: {}", s)),
        }
    }
}

/// PutObjectACL 操作输入
#[derive(Debug, Clone)]
pub struct PutObjectAclInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
    /// 对象 ACL 权限
    pub acl: ObjectAcl,
    /// 版本 ID（可选）
    /// 在开启版本控制的 Bucket 中，指定此参数可以设置指定版本 Object 的 ACL
    pub version_id: Option<String>,
}

impl PutObjectAclInput {
    /// 创建新的 PutObjectAclInput 构建器
    pub fn builder() -> PutObjectAclInputBuilder {
        PutObjectAclInputBuilder::default()
    }
}

/// PutObjectAclInput 构建器
#[derive(Debug, Default)]
pub struct PutObjectAclInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    acl: Option<ObjectAcl>,
    version_id: Option<String>,
}

impl PutObjectAclInputBuilder {
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

    /// 设置对象 ACL 权限
    pub fn acl(mut self, acl: ObjectAcl) -> Self {
        self.acl = Some(acl);
        self
    }

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 构建 PutObjectAclInput
    pub fn build(self) -> Result<PutObjectAclInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let acl = self.acl.ok_or("acl is required")?;

        Ok(PutObjectAclInput {
            bucket,
            key,
            acl,
            version_id: self.version_id,
        })
    }
}

/// PutObjectACL 操作输出
#[derive(Debug, Default)]
pub struct PutObjectAclOutput {
    /// 请求 ID
    pub request_id: Option<String>,
    /// 版本 ID（仅在开启版本控制的 Bucket 中返回）
    pub version_id: Option<String>,
}
