//! HeadObject Input/Output 类型定义

use std::collections::HashMap;

/// HeadObject 操作输入
#[derive(Debug, Clone)]
pub struct HeadObjectInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
    /// 版本 ID
    pub version_id: Option<String>,
    /// 如果传入参数中的时间早于实际修改时间，则返回 200 OK 和 Object Meta；否则返回 304 Not Modified
    pub if_modified_since: Option<String>,
    /// 如果传入参数中的时间等于或者晚于文件实际修改时间，则返回 200 OK 和 Object Meta；否则返回 412 Precondition Failed
    pub if_unmodified_since: Option<String>,
    /// 如果传入期望的 ETag 和 Object 的 ETag 匹配，则返回 200 OK 和 Object Meta；否则返回 412 Precondition Failed
    pub if_match: Option<String>,
    /// 如果传入期望的 ETag 值和 Object 的 ETag 不匹配，则返回 200 OK 和 Object Meta；否则返回 304 Not Modified
    pub if_none_match: Option<String>,
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
    version_id: Option<String>,
    if_modified_since: Option<String>,
    if_unmodified_since: Option<String>,
    if_match: Option<String>,
    if_none_match: Option<String>,
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

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 设置 If-Modified-Since 条件
    pub fn if_modified_since(mut self, if_modified_since: impl Into<String>) -> Self {
        self.if_modified_since = Some(if_modified_since.into());
        self
    }

    /// 设置 If-Unmodified-Since 条件
    pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<String>) -> Self {
        self.if_unmodified_since = Some(if_unmodified_since.into());
        self
    }

    /// 设置 If-Match 条件
    pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
        self.if_match = Some(if_match.into());
        self
    }

    /// 设置 If-None-Match 条件
    pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
        self.if_none_match = Some(if_none_match.into());
        self
    }

    /// 构建 HeadObjectInput
    pub fn build(self) -> Result<HeadObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(HeadObjectInput {
            bucket,
            key,
            version_id: self.version_id,
            if_modified_since: self.if_modified_since,
            if_unmodified_since: self.if_unmodified_since,
            if_match: self.if_match,
            if_none_match: self.if_none_match,
        })
    }
}

/// Object 类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    /// 普通文件
    Normal,
    /// 追加上传文件
    Appendable,
    /// 分片上传文件
    Multipart,
    /// 未知类型
    Unknown,
}

impl From<&str> for ObjectType {
    fn from(s: &str) -> Self {
        match s {
            "Normal" => ObjectType::Normal,
            "Appendable" => ObjectType::Appendable,
            "Multipart" => ObjectType::Multipart,
            _ => ObjectType::Unknown,
        }
    }
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Normal => write!(f, "Normal"),
            ObjectType::Appendable => write!(f, "Appendable"),
            ObjectType::Multipart => write!(f, "Multipart"),
            ObjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Restore 状态信息
#[derive(Debug, Clone)]
pub struct RestoreInfo {
    /// 是否正在进行 restore 请求
    pub ongoing: bool,
    /// Restore 完成后的过期时间（仅当 ongoing 为 false 时存在）
    pub expiry_date: Option<String>,
}

impl RestoreInfo {
    /// 从 x-oss-restore 头部解析 RestoreInfo
    pub fn from_header(value: &str) -> Self {
        if value.contains("ongoing-request=\"true\"") {
            Self {
                ongoing: true,
                expiry_date: None,
            }
        } else if value.contains("ongoing-request=\"false\"") {
            let expiry_date = value
                .split("expiry-date=\"")
                .nth(1)
                .and_then(|s| s.strip_suffix('"'))
                .map(|s| s.to_string());
            Self {
                ongoing: false,
                expiry_date,
            }
        } else {
            Self {
                ongoing: false,
                expiry_date: None,
            }
        }
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
    /// Object 类型
    pub object_type: Option<ObjectType>,
    /// 版本 ID
    pub version_id: Option<String>,
    /// 服务端加密算法
    pub server_side_encryption: Option<String>,
    /// 服务端加密 Key ID
    pub server_side_encryption_key_id: Option<String>,
    /// CRC64 校验值
    pub hash_crc64ecma: Option<u64>,
    /// Content-MD5
    pub content_md5: Option<String>,
    /// 对于 Appendable 类型的 Object，下一次请求应当提供的 position
    pub next_append_position: Option<u64>,
    /// 对于已封存的 Appendable 类型的 Object，执行 seal 操作的时间
    pub sealed_time: Option<String>,
    /// Restore 状态信息
    pub restore: Option<RestoreInfo>,
    /// 对象关联的标签个数
    pub tagging_count: Option<u32>,
    /// 过期时间
    pub expiration: Option<String>,
    /// 转储时间
    pub transition_time: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// 用户自定义元数据
    pub metadata: HashMap<String, String>,
    /// Cache-Control
    pub cache_control: Option<String>,
    /// Content-Disposition
    pub content_disposition: Option<String>,
    /// Content-Encoding
    pub content_encoding: Option<String>,
    /// Expires
    pub expires: Option<String>,
}

/// HeadObject 结果状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeadObjectStatus {
    /// 成功获取 Object 元数据
    Ok,
    /// Object 未修改（304 Not Modified）
    NotModified,
    /// 前置条件失败（412 Precondition Failed）
    PreconditionFailed,
}

impl HeadObjectOutput {
    /// 获取结果状态
    pub fn status(&self) -> HeadObjectStatus {
        HeadObjectStatus::Ok
    }
}
