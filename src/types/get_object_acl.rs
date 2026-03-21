//! GetObjectACL Input/Output 类型定义
//!
//! 获取 Object 的访问权限（ACL）

use serde::Deserialize;

/// Object ACL 权限类型
/// 重导出自 put_object_acl 模块
pub use super::put_object_acl::ObjectAcl as ObjectAclPermission;

/// GetObjectACL 操作输入
#[derive(Debug, Clone)]
pub struct GetObjectAclInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
    /// 版本 ID（可选）
    /// 在开启版本控制的 Bucket 中，指定此参数可以获取指定版本 Object 的 ACL
    pub version_id: Option<String>,
}

impl GetObjectAclInput {
    /// 创建新的 GetObjectAclInput 构建器
    pub fn builder() -> GetObjectAclInputBuilder {
        GetObjectAclInputBuilder::default()
    }
}

/// GetObjectAclInput 构建器
#[derive(Debug, Default)]
pub struct GetObjectAclInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

impl GetObjectAclInputBuilder {
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

    /// 构建 GetObjectAclInput
    pub fn build(self) -> Result<GetObjectAclInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(GetObjectAclInput {
            bucket,
            key,
            version_id: self.version_id,
        })
    }
}

/// GetObjectACL 操作输出
#[derive(Debug)]
pub struct GetObjectAclOutput {
    /// Object 拥有者信息
    pub owner: Owner,
    /// Object 的 ACL 权限
    pub grant: ObjectAclPermission,
    /// 请求 ID
    pub request_id: Option<String>,
    /// 版本 ID（仅在开启版本控制的 Bucket 中返回）
    pub version_id: Option<String>,
}

/// Object 拥有者信息
#[derive(Debug, Clone, Deserialize)]
pub struct Owner {
    /// 用户 ID
    pub id: String,
    /// 显示名称（与用户 ID 一致）
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

impl Default for Owner {
    fn default() -> Self {
        Self {
            id: String::new(),
            display_name: String::new(),
        }
    }
}

/// ACL 响应 XML 结构
#[derive(Debug, Deserialize)]
pub struct AccessControlPolicy {
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: AccessControlList,
}

/// ACL 列表
#[derive(Debug, Deserialize)]
pub struct AccessControlList {
    #[serde(rename = "Grant")]
    pub grant: String,
}

impl AccessControlPolicy {
    /// 从 XML 解析 ACL 响应
    pub fn parse(xml: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(xml)
    }
}
