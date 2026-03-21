//! ListBuckets（GetService）Input/Output 类型定义

use serde::{Deserialize, Serialize};

/// ListBuckets 操作输入
#[derive(Debug, Clone, Default)]
pub struct ListBucketsInput {
    /// 限定返回的 Bucket 名称必须以 prefix 作为前缀
    pub prefix: Option<String>,
    /// 设定结果从 marker 之后按字母排序的第一个开始返回
    pub marker: Option<String>,
    /// 限定此次返回 Bucket 的最大个数（1~1000）
    pub max_keys: Option<i32>,
    /// 资源组 ID
    pub resource_group_id: Option<String>,
}

impl ListBucketsInput {
    /// 创建新的 ListBucketsInput 构建器
    pub fn builder() -> ListBucketsInputBuilder {
        ListBucketsInputBuilder::default()
    }
}

/// ListBucketsInput 构建器
#[derive(Debug, Default)]
pub struct ListBucketsInputBuilder {
    prefix: Option<String>,
    marker: Option<String>,
    max_keys: Option<i32>,
    resource_group_id: Option<String>,
}

impl ListBucketsInputBuilder {
    /// 设置前缀
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
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

    /// 设置资源组 ID
    pub fn resource_group_id(mut self, resource_group_id: impl Into<String>) -> Self {
        self.resource_group_id = Some(resource_group_id.into());
        self
    }

    /// 构建 ListBucketsInput
    pub fn build(self) -> ListBucketsInput {
        ListBucketsInput {
            prefix: self.prefix,
            marker: self.marker,
            max_keys: self.max_keys,
            resource_group_id: self.resource_group_id,
        }
    }
}

/// ListBuckets 操作输出
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "ListAllMyBucketsResult")]
pub struct ListBucketsOutput {
    /// 本次查询结果的前缀
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// 表示本次 ListBuckets 的起点
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
    /// 响应请求内返回结果的最大个数
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<i32>,
    /// 是否所有的结果都已经返回
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<bool>,
    /// 用于继续查询时给 marker 赋值
    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,
    /// Bucket 拥有者信息
    #[serde(rename = "Owner")]
    pub owner: Owner,
    /// Bucket 列表
    #[serde(rename = "Buckets")]
    pub buckets: Buckets,
}

/// Bucket 拥有者信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Owner {
    /// Bucket 拥有者的用户 ID
    #[serde(rename = "ID")]
    pub id: String,
    /// Bucket 拥有者的名称（目前和 ID 一致）
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

/// Bucket 列表容器
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Buckets {
    /// Bucket 信息列表
    #[serde(rename = "Bucket", default)]
    pub bucket: Vec<BucketInfo>,
}

/// Bucket 信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucketInfo {
    /// Bucket 名称
    #[serde(rename = "Name")]
    pub name: String,
    /// Bucket 创建时间
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// OSS 专用 Region ID
    #[serde(rename = "Location")]
    pub location: String,
    /// 外网访问域名
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: Option<String>,
    /// 内网访问域名
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: Option<String>,
    /// 阿里云通用 Region ID
    #[serde(rename = "Region")]
    pub region: Option<String>,
    /// Bucket 存储类型
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,
    /// Bucket 所属资源组 ID
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: Option<String>,
}
