//! ListObjects Input/Output 类型定义

use serde::{Deserialize, Serialize};

/// ListObjects 操作输入
#[derive(Debug, Clone)]
pub struct ListObjectsInput {
    /// 存储桶名称
    pub bucket: String,
    /// 限制返回结果的前缀
    pub prefix: Option<String>,
    /// 分隔符
    pub delimiter: Option<String>,
    /// 标记位置（分页用）
    pub marker: Option<String>,
    /// 最大返回数量
    pub max_keys: Option<i32>,
    /// 编码类型
    pub encoding_type: Option<String>,
}

impl ListObjectsInput {
    /// 创建新的 ListObjectsInput 构建器
    pub fn builder() -> ListObjectsInputBuilder {
        ListObjectsInputBuilder::default()
    }
}

/// ListObjectsInput 构建器
#[derive(Debug, Default)]
pub struct ListObjectsInputBuilder {
    bucket: Option<String>,
    prefix: Option<String>,
    delimiter: Option<String>,
    marker: Option<String>,
    max_keys: Option<i32>,
    encoding_type: Option<String>,
}

impl ListObjectsInputBuilder {
    /// 设置存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 设置前缀
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// 设置分隔符
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }

    /// 设置标记位置
    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.marker = Some(marker.into());
        self
    }

    /// 设置最大返回数量
    pub fn max_keys(mut self, max_keys: i32) -> Self {
        self.max_keys = Some(max_keys);
        self
    }

    /// 设置编码类型
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.encoding_type = Some(encoding_type.into());
        self
    }

    /// 构建 ListObjectsInput
    pub fn build(self) -> Result<ListObjectsInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;

        Ok(ListObjectsInput {
            bucket,
            prefix: self.prefix,
            delimiter: self.delimiter,
            marker: self.marker,
            max_keys: self.max_keys,
            encoding_type: self.encoding_type,
        })
    }
}

/// ListObjects 操作输出
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListObjectsOutput {
    /// 存储桶名称
    #[serde(rename = "Name")]
    pub name: String,
    /// 前缀
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// 标记位置
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
    /// 分隔符
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    /// 编码类型
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,
    /// 最大返回数量
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<i32>,
    /// 是否截断
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    /// 下一个标记位置
    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,
    /// 对象列表
    #[serde(rename = "Contents")]
    pub contents: Option<Vec<Object>>,
    /// 公共前缀列表
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// 键数量
    #[serde(rename = "KeyCount")]
    pub key_count: Option<i32>,
}

/// 对象信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Object {
    /// 对象键
    #[serde(rename = "Key")]
    pub key: String,
    /// 最后修改时间
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    /// ETag
    #[serde(rename = "ETag")]
    pub etag: String,
    /// Content-Type
    #[serde(rename = "Type")]
    pub content_type: Option<String>,
    /// 大小
    #[serde(rename = "Size")]
    pub size: i64,
    /// 存储类型
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
    /// 所有者
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}

/// 所有者信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Owner {
    /// ID
    #[serde(rename = "ID")]
    pub id: String,
    /// 显示名称
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

/// 公共前缀
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommonPrefix {
    /// 前缀
    #[serde(rename = "Prefix")]
    pub prefix: String,
}