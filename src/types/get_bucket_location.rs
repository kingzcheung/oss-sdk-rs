//! GetBucketLocation 操作 Input/Output 类型定义

use serde::{Deserialize, Serialize};

/// GetBucketLocation 操作输入
#[derive(Debug, Clone, Default)]
pub struct GetBucketLocationInput {
    /// Bucket 名称
    pub bucket: String,
}

impl GetBucketLocationInput {
    /// 创建新的 GetBucketLocationInput 构建器
    pub fn builder() -> GetBucketLocationInputBuilder {
        GetBucketLocationInputBuilder::default()
    }
}

/// GetBucketLocationInput 构建器
#[derive(Debug, Default)]
pub struct GetBucketLocationInputBuilder {
    bucket: Option<String>,
}

impl GetBucketLocationInputBuilder {
    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 构建 GetBucketLocationInput
    pub fn build(self) -> Result<GetBucketLocationInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        Ok(GetBucketLocationInput { bucket })
    }
}

/// GetBucketLocation 操作输出
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "LocationConstraint")]
pub struct GetBucketLocationOutput {
    /// Bucket 所在地域，格式为 OSS 专用 Region ID
    #[serde(rename = "$value")]
    pub location_constraint: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_get_bucket_location_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<LocationConstraint>oss-cn-hangzhou</LocationConstraint>"#;

        let output: GetBucketLocationOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.location_constraint, "oss-cn-hangzhou");
    }

    #[test]
    fn test_deserialize_get_bucket_location_shanghai() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<LocationConstraint>oss-cn-shanghai</LocationConstraint>"#;

        let output: GetBucketLocationOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.location_constraint, "oss-cn-shanghai");
    }
}
