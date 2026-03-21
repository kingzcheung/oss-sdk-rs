//! GetBucketInfo 操作 Input/Output 类型定义

use serde::{Deserialize, Serialize};

/// GetBucketInfo 操作输入
#[derive(Debug, Clone, Default)]
pub struct GetBucketInfoInput {
    /// Bucket 名称
    pub bucket: String,
}

impl GetBucketInfoInput {
    /// 创建新的 GetBucketInfoInput 构建器
    pub fn builder() -> GetBucketInfoInputBuilder {
        GetBucketInfoInputBuilder::default()
    }
}

/// GetBucketInfoInput 构建器
#[derive(Debug, Default)]
pub struct GetBucketInfoInputBuilder {
    bucket: Option<String>,
}

impl GetBucketInfoInputBuilder {
    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 构建 GetBucketInfoInput
    pub fn build(self) -> Result<GetBucketInfoInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        Ok(GetBucketInfoInput { bucket })
    }
}

/// GetBucketInfo 操作输出
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "BucketInfo")]
pub struct GetBucketInfoOutput {
    /// Bucket 信息容器
    #[serde(rename = "Bucket")]
    pub bucket: BucketInfoDetail,
}

/// Bucket 详细信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucketInfoDetail {
    /// Bucket 的访问跟踪状态
    #[serde(rename = "AccessMonitor", default)]
    pub access_monitor: Option<String>,

    /// Bucket 的创建时间，格式为 UTC 时间
    #[serde(rename = "CreationDate")]
    pub creation_date: String,

    /// 外网 Endpoint
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: String,

    /// 内网 Endpoint
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: String,

    /// Bucket 所在地域，格式为 OSS 专用 Region ID
    #[serde(rename = "Location")]
    pub location: String,

    /// Bucket 的存储类型
    /// 有效值：Standard、IA、Archive、ColdArchive
    #[serde(rename = "StorageClass")]
    pub storage_class: String,

    /// Bucket 的传输加速状态
    /// 有效值：Enabled、Disabled
    #[serde(rename = "TransferAcceleration", default)]
    pub transfer_acceleration: Option<String>,

    /// Bucket 的跨区域复制状态
    /// 有效值：Enabled、Disabled
    #[serde(rename = "CrossRegionReplication", default)]
    pub cross_region_replication: Option<String>,

    /// Bucket 名称
    #[serde(rename = "Name")]
    pub name: String,

    /// Bucket 所属的资源组 ID
    #[serde(rename = "ResourceGroupId", default)]
    pub resource_group_id: Option<String>,

    /// Bucket 拥有者信息
    #[serde(rename = "Owner")]
    pub owner: OwnerInfo,

    /// Bucket 读写权限（ACL）信息
    #[serde(rename = "AccessControlList")]
    pub access_control_list: AccessControlListInfo,

    /// 备注
    #[serde(rename = "Comment", default)]
    pub comment: Option<String>,

    /// Bucket 的数据容灾类型
    /// 有效值：LRS、ZRS
    #[serde(rename = "DataRedundancyType", default)]
    pub data_redundancy_type: Option<String>,

    /// Bucket 的版本控制状态
    /// 有效值：Enabled、Suspended
    #[serde(rename = "Versioning", default)]
    pub versioning: Option<String>,

    /// 服务端加密方式
    #[serde(rename = "ServerSideEncryptionRule", default)]
    pub server_side_encryption_rule: Option<ServerSideEncryptionRule>,

    /// 日志信息
    #[serde(rename = "BucketPolicy", default)]
    pub bucket_policy: Option<BucketPolicyInfo>,

    /// Bucket 阻止公共访问的配置信息
    /// true：开启阻止公共访问
    /// false：关闭阻止公共访问
    #[serde(rename = "BlockPublicAccess", default)]
    pub block_public_access: Option<bool>,
}

/// Bucket 拥有者信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OwnerInfo {
    /// Bucket 拥有者的名称（目前和用户 ID 一致）
    #[serde(rename = "DisplayName")]
    pub display_name: String,

    /// Bucket 拥有者的用户 ID
    #[serde(rename = "ID")]
    pub id: String,
}

/// Bucket 读写权限（ACL）信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessControlListInfo {
    /// Bucket 的 ACL 权限
    /// 有效值：private、public-read、public-read-write
    #[serde(rename = "Grant")]
    pub grant: String,
}

/// 服务端加密方式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerSideEncryptionRule {
    /// 服务端默认加密方式
    /// 有效值：KMS、AES256
    #[serde(rename = "SSEAlgorithm", default)]
    pub sse_algorithm: Option<String>,

    /// 当前使用的 KMS 密钥 ID
    #[serde(rename = "KMSMasterKeyID", default)]
    pub kms_master_key_id: Option<String>,

    /// Object 的加密算法
    #[serde(rename = "KMSDataEncryption", default)]
    pub kms_data_encryption: Option<String>,
}

/// 日志信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucketPolicyInfo {
    /// 存储日志记录的 Bucket 名称
    #[serde(rename = "LogBucket", default)]
    pub log_bucket: Option<String>,

    /// 存储日志文件的目录
    #[serde(rename = "LogPrefix", default)]
    pub log_prefix: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_get_bucket_info_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<BucketInfo>
  <Bucket>
    <AccessMonitor>Enabled</AccessMonitor>
    <CreationDate>2013-07-31T10:56:21.000Z</CreationDate>
    <ExtranetEndpoint>oss-cn-hangzhou.aliyuncs.com</ExtranetEndpoint>
    <IntranetEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</IntranetEndpoint>
    <Location>oss-cn-hangzhou</Location>
    <StorageClass>Standard</StorageClass>
    <TransferAcceleration>Disabled</TransferAcceleration>
    <CrossRegionReplication>Disabled</CrossRegionReplication>
    <Name>oss-example</Name>
    <ResourceGroupId>rg-aek27tc********</ResourceGroupId>
    <Owner>
      <DisplayName>username</DisplayName>
      <ID>27183473914****</ID>
    </Owner>
    <AccessControlList>
      <Grant>private</Grant>
    </AccessControlList>  
    <Comment>test</Comment>
    <BucketPolicy>
      <LogBucket>examplebucket</LogBucket>
      <LogPrefix>log/</LogPrefix>
    </BucketPolicy>
    <BlockPublicAccess>true</BlockPublicAccess>
  </Bucket>
</BucketInfo>"#;

        let output: GetBucketInfoOutput = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(output.bucket.name, "oss-example");
        assert_eq!(output.bucket.creation_date, "2013-07-31T10:56:21.000Z");
        assert_eq!(
            output.bucket.extranet_endpoint,
            "oss-cn-hangzhou.aliyuncs.com"
        );
        assert_eq!(
            output.bucket.intranet_endpoint,
            "oss-cn-hangzhou-internal.aliyuncs.com"
        );
        assert_eq!(output.bucket.location, "oss-cn-hangzhou");
        assert_eq!(output.bucket.storage_class, "Standard");
        assert_eq!(output.bucket.access_monitor, Some("Enabled".to_string()));
        assert_eq!(
            output.bucket.transfer_acceleration,
            Some("Disabled".to_string())
        );
        assert_eq!(
            output.bucket.cross_region_replication,
            Some("Disabled".to_string())
        );
        assert_eq!(
            output.bucket.resource_group_id,
            Some("rg-aek27tc********".to_string())
        );
        assert_eq!(output.bucket.owner.display_name, "username");
        assert_eq!(output.bucket.owner.id, "27183473914****");
        assert_eq!(output.bucket.access_control_list.grant, "private");
        assert_eq!(output.bucket.comment, Some("test".to_string()));

        let policy = output.bucket.bucket_policy.unwrap();
        assert_eq!(policy.log_bucket, Some("examplebucket".to_string()));
        assert_eq!(policy.log_prefix, Some("log/".to_string()));

        assert_eq!(output.bucket.block_public_access, Some(true));
    }
}
