//! PutObject Input/Output 类型定义

use crate::primitives::ByteStream;

/// 存储类型
#[derive(Debug, Clone, Copy, Default)]
pub enum StorageClass {
    #[default]
    Standard,
    IA,
    Archive,
    ColdArchive,
    DeepColdArchive,
}

impl std::fmt::Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageClass::Standard => write!(f, "Standard"),
            StorageClass::IA => write!(f, "IA"),
            StorageClass::Archive => write!(f, "Archive"),
            StorageClass::ColdArchive => write!(f, "ColdArchive"),
            StorageClass::DeepColdArchive => write!(f, "DeepColdArchive"),
        }
    }
}

/// 对象 ACL
#[derive(Debug, Clone, Copy, Default)]
pub enum ObjectAcl {
    #[default]
    Default,
    Private,
    PublicRead,
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

/// Content-Disposition 类型
#[derive(Debug, Clone)]
pub enum ContentDisposition {
    Inline,
    Attachment,
    AttachmentWithFileName(String),
}

impl std::fmt::Display for ContentDisposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentDisposition::Inline => write!(f, "inline"),
            ContentDisposition::Attachment => write!(f, "attachment"),
            ContentDisposition::AttachmentWithFileName(filename) => {
                write!(f, "attachment; filename=\"{}\"", filename)
            }
        }
    }
}

/// Content-Encoding 类型
#[derive(Debug, Clone, Copy)]
pub enum ContentEncoding {
    Identity,
    Gzip,
    Compress,
    Deflate,
    Br,
}

impl std::fmt::Display for ContentEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentEncoding::Identity => write!(f, "identity"),
            ContentEncoding::Gzip => write!(f, "gzip"),
            ContentEncoding::Compress => write!(f, "compress"),
            ContentEncoding::Deflate => write!(f, "deflate"),
            ContentEncoding::Br => write!(f, "br"),
        }
    }
}

/// PutObject 操作输入
#[derive(Debug)]
pub struct PutObjectInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
    /// 对象内容
    pub body: ByteStream,
    /// Content-Type
    pub content_type: Option<String>,
    /// Content-Disposition
    pub content_disposition: Option<ContentDisposition>,
    /// Content-Encoding
    pub content_encoding: Option<ContentEncoding>,
    /// 存储类型
    pub storage_class: Option<StorageClass>,
    /// 对象 ACL
    pub acl: Option<ObjectAcl>,
    /// 是否禁止覆盖
    pub forbid_overwrite: bool,
    /// 自定义元数据
    pub metadata: std::collections::HashMap<String, String>,
}

impl PutObjectInput {
    /// 创建新的 PutObjectInput 构建器
    pub fn builder() -> PutObjectInputBuilder {
        PutObjectInputBuilder::default()
    }
}

/// PutObjectInput 构建器
#[derive(Debug, Default)]
pub struct PutObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    body: Option<ByteStream>,
    content_type: Option<String>,
    content_disposition: Option<ContentDisposition>,
    content_encoding: Option<ContentEncoding>,
    storage_class: Option<StorageClass>,
    acl: Option<ObjectAcl>,
    forbid_overwrite: bool,
    metadata: std::collections::HashMap<String, String>,
}

impl PutObjectInputBuilder {
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

    /// 设置对象内容
    pub fn body(mut self, body: impl Into<ByteStream>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 设置对象内容（从 Vec<u8>）
    pub fn body_from_vec(mut self, body: Vec<u8>) -> Self {
        self.body = Some(ByteStream::from_vec(body));
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// 设置 Content-Disposition
    pub fn content_disposition(mut self, disposition: ContentDisposition) -> Self {
        self.content_disposition = Some(disposition);
        self
    }

    /// 设置 Content-Encoding
    pub fn content_encoding(mut self, encoding: ContentEncoding) -> Self {
        self.content_encoding = Some(encoding);
        self
    }

    /// 设置存储类型
    pub fn storage_class(mut self, storage_class: StorageClass) -> Self {
        self.storage_class = Some(storage_class);
        self
    }

    /// 设置对象 ACL
    pub fn acl(mut self, acl: ObjectAcl) -> Self {
        self.acl = Some(acl);
        self
    }

    /// 设置是否禁止覆盖
    pub fn forbid_overwrite(mut self, forbid: bool) -> Self {
        self.forbid_overwrite = forbid;
        self
    }

    /// 添加自定义元数据
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 构建 PutObjectInput
    pub fn build(self) -> Result<PutObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let body = self.body.unwrap_or_default();

        Ok(PutObjectInput {
            bucket,
            key,
            body,
            content_type: self.content_type,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            storage_class: self.storage_class,
            acl: self.acl,
            forbid_overwrite: self.forbid_overwrite,
            metadata: self.metadata,
        })
    }
}

/// PutObject 操作输出
#[derive(Debug, Default)]
pub struct PutObjectOutput {
    /// ETag
    pub etag: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
}
