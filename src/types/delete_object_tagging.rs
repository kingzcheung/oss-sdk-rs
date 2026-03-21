//! DeleteObjectTagging 操作的类型定义
//!
//! 用于删除对象（Object）的标签（Tagging）信息

/// DeleteObjectTagging 输入
#[derive(Debug, Clone)]
pub struct DeleteObjectTaggingInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// 版本 ID
    pub version_id: Option<String>,
}

impl DeleteObjectTaggingInput {
    /// 创建新的 DeleteObjectTagging 输入
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// DeleteObjectTagging 输入构建器
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteObjectTaggingInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

#[allow(dead_code)]
impl DeleteObjectTaggingInputBuilder {
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

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 构建输入
    pub fn build(self) -> Result<DeleteObjectTaggingInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(DeleteObjectTaggingInput {
            bucket,
            key,
            version_id: self.version_id,
        })
    }
}

/// DeleteObjectTagging 输出
#[derive(Debug, Clone)]
pub struct DeleteObjectTaggingOutput {
    /// 请求 ID
    pub request_id: String,
    /// 版本 ID
    pub version_id: Option<String>,
}

impl DeleteObjectTaggingOutput {
    /// 创建新的输出
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            version_id: None,
        }
    }

    /// 设置版本 ID
    pub fn with_version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_object_tagging_input() {
        let input = DeleteObjectTaggingInput::new("my-bucket", "my-object");
        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object");
        assert_eq!(input.version_id, None);
    }

    #[test]
    fn test_delete_object_tagging_input_with_version() {
        let input = DeleteObjectTaggingInput::new("my-bucket", "my-object").version_id("v1");
        assert_eq!(input.version_id, Some("v1".to_string()));
    }

    #[test]
    fn test_delete_object_tagging_input_builder() {
        let input = DeleteObjectTaggingInputBuilder::default()
            .bucket("my-bucket")
            .key("my-object")
            .version_id("v1")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object");
        assert_eq!(input.version_id, Some("v1".to_string()));
    }

    #[test]
    fn test_delete_object_tagging_input_builder_missing_bucket() {
        let result = DeleteObjectTaggingInputBuilder::default()
            .key("my-object")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_object_tagging_input_builder_missing_key() {
        let result = DeleteObjectTaggingInputBuilder::default()
            .bucket("my-bucket")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_object_tagging_output() {
        let output = DeleteObjectTaggingOutput::new("request-id-123");
        assert_eq!(output.request_id, "request-id-123");
        assert_eq!(output.version_id, None);
    }

    #[test]
    fn test_delete_object_tagging_output_with_version() {
        let output = DeleteObjectTaggingOutput::new("request-id-123").with_version_id("v1");
        assert_eq!(output.version_id, Some("v1".to_string()));
    }
}
