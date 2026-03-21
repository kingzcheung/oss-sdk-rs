//! AppendObject 操作 Input/Output 类型定义

/// AppendObject 操作输入
#[derive(Debug, Clone, Default)]
pub struct AppendObjectInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// 追加位置
    /// 首次追加必须为 0，后续追加为当前 Object 大小
    pub position: u64,
    /// 追加的内容
    pub body: Vec<u8>,
    /// Content-Type
    pub content_type: Option<String>,
    /// 缓存控制
    pub cache_control: Option<String>,
    /// 内容处置
    pub content_disposition: Option<String>,
    /// Content-MD5
    pub content_md5: Option<String>,
    /// 过期时间
    pub expires: Option<String>,
    /// 服务端加密方式
    pub server_side_encryption: Option<String>,
    /// Object ACL
    pub object_acl: Option<String>,
    /// 存储类型（仅首次追加有效）
    pub storage_class: Option<String>,
    /// 自定义元数据
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Object 标签（仅首次追加有效）
    pub tagging: Option<String>,
}

impl AppendObjectInput {
    /// 创建新的 AppendObjectInput 构建器
    pub fn builder() -> AppendObjectInputBuilder {
        AppendObjectInputBuilder::default()
    }
}

/// AppendObjectInput 构建器
#[derive(Debug, Default)]
pub struct AppendObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    position: Option<u64>,
    body: Option<Vec<u8>>,
    content_type: Option<String>,
    cache_control: Option<String>,
    content_disposition: Option<String>,
    content_md5: Option<String>,
    expires: Option<String>,
    server_side_encryption: Option<String>,
    object_acl: Option<String>,
    storage_class: Option<String>,
    metadata: Option<std::collections::HashMap<String, String>>,
    tagging: Option<String>,
}

impl AppendObjectInputBuilder {
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

    /// 设置追加位置
    pub fn position(mut self, position: u64) -> Self {
        self.position = Some(position);
        self
    }

    /// 设置追加内容
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// 设置缓存控制
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.cache_control = Some(cache_control.into());
        self
    }

    /// 设置内容处置
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.content_disposition = Some(content_disposition.into());
        self
    }

    /// 设置 Content-MD5
    pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
        self.content_md5 = Some(content_md5.into());
        self
    }

    /// 设置过期时间
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }

    /// 设置服务端加密方式
    pub fn server_side_encryption(mut self, encryption: impl Into<String>) -> Self {
        self.server_side_encryption = Some(encryption.into());
        self
    }

    /// 设置 Object ACL
    pub fn object_acl(mut self, acl: impl Into<String>) -> Self {
        self.object_acl = Some(acl.into());
        self
    }

    /// 设置存储类型
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.storage_class = Some(storage_class.into());
        self
    }

    /// 设置自定义元数据
    pub fn metadata(mut self, metadata: std::collections::HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// 设置 Object 标签
    pub fn tagging(mut self, tagging: impl Into<String>) -> Self {
        self.tagging = Some(tagging.into());
        self
    }

    /// 构建 AppendObjectInput
    pub fn build(self) -> Result<AppendObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let position = self.position.ok_or("position is required")?;
        let body = self.body.ok_or("body is required")?;

        Ok(AppendObjectInput {
            bucket,
            key,
            position,
            body,
            content_type: self.content_type,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_md5: self.content_md5,
            expires: self.expires,
            server_side_encryption: self.server_side_encryption,
            object_acl: self.object_acl,
            storage_class: self.storage_class,
            metadata: self.metadata,
            tagging: self.tagging,
        })
    }
}

/// AppendObject 操作输出
#[derive(Debug, Clone, Default)]
pub struct AppendObjectOutput {
    /// ETag
    pub etag: Option<String>,
    /// 下一次追加的位置
    pub next_append_position: Option<u64>,
    /// CRC64 值
    pub hash_crc64ecma: Option<u64>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// 版本 ID（开启版本控制时返回）
    pub version_id: Option<String>,
}

/// Object ACL 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ObjectAcl {
    /// 默认（继承 Bucket 权限）
    Default,
    /// 私有
    Private,
    /// 公共读
    PublicRead,
    /// 公共读写
    PublicReadWrite,
}

impl std::fmt::Display for ObjectAcl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectAcl::Default => write!(f, "default"),
            ObjectAcl::Private => write!(f, "private"),
            ObjectAcl::PublicRead => write!(f, "public-read"),
            ObjectAcl::PublicReadWrite => write!(f, "public-read-write"),
        }
    }
}

/// 存储类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StorageClass {
    /// 标准存储
    Standard,
    /// 低频访问
    IA,
    /// 归档存储
    Archive,
}

impl std::fmt::Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageClass::Standard => write!(f, "Standard"),
            StorageClass::IA => write!(f, "IA"),
            StorageClass::Archive => write!(f, "Archive"),
        }
    }
}

/// 服务端加密方式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ServerSideEncryption {
    /// AES256
    Aes256,
    /// KMS
    Kms,
    /// SM4
    Sm4,
}

impl std::fmt::Display for ServerSideEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerSideEncryption::Aes256 => write!(f, "AES256"),
            ServerSideEncryption::Kms => write!(f, "KMS"),
            ServerSideEncryption::Sm4 => write!(f, "SM4"),
        }
    }
}
