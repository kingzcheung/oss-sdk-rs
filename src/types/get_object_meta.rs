//! GetObjectMeta Input/Output 类型定义
//!
//! 获取文件的元数据信息，包括 ETag、Size、LastModified 信息，不返回文件内容。

/// GetObjectMeta 操作输入
#[derive(Debug, Clone)]
pub struct GetObjectMetaInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
    /// 版本 ID
    pub version_id: Option<String>,
}

impl GetObjectMetaInput {
    /// 创建新的 GetObjectMetaInput 构建器
    pub fn builder() -> GetObjectMetaInputBuilder {
        GetObjectMetaInputBuilder::default()
    }
}

/// GetObjectMetaInput 构建器
#[derive(Debug, Default)]
pub struct GetObjectMetaInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

impl GetObjectMetaInputBuilder {
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

    /// 构建 GetObjectMetaInput
    pub fn build(self) -> Result<GetObjectMetaInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(GetObjectMetaInput {
            bucket,
            key,
            version_id: self.version_id,
        })
    }
}

/// GetObjectMeta 操作输出
#[derive(Debug, Default)]
pub struct GetObjectMetaOutput {
    /// Object 的文件大小，单位为字节
    pub content_length: Option<u64>,
    /// ETag（entity tag），用于标识 Object 的内容
    pub etag: Option<String>,
    /// Object 最后一次修改时间
    pub last_modified: Option<String>,
    /// Object 通过生命周期规则转储为冷归档或者深度冷归档存储类型的时间
    pub transition_time: Option<String>,
    /// Object 的最后一次访问时间（开启访问跟踪后返回）
    pub last_access_time: Option<String>,
    /// Object 的版本 ID（仅当请求指定 versionId 时返回）
    pub version_id: Option<String>,
    /// 已经处于 Sealed 状态的 Append 文件执行 seal 操作的时间
    pub sealed_time: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = GetObjectMetaInput::builder()
            .bucket("my-bucket")
            .key("my-object.txt")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object.txt");
        assert_eq!(input.version_id, None);
    }

    #[test]
    fn test_input_builder_with_version() {
        let input = GetObjectMetaInput::builder()
            .bucket("my-bucket")
            .key("my-object.txt")
            .version_id("123456")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object.txt");
        assert_eq!(input.version_id, Some("123456".to_string()));
    }

    #[test]
    fn test_input_builder_missing_bucket() {
        let result = GetObjectMetaInput::builder().key("my-object.txt").build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "bucket is required");
    }

    #[test]
    fn test_input_builder_missing_key() {
        let result = GetObjectMetaInput::builder().bucket("my-bucket").build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "key is required");
    }

    #[test]
    fn test_output_default() {
        let output = GetObjectMetaOutput::default();

        assert_eq!(output.content_length, None);
        assert_eq!(output.etag, None);
        assert_eq!(output.last_modified, None);
        assert_eq!(output.transition_time, None);
        assert_eq!(output.last_access_time, None);
        assert_eq!(output.version_id, None);
        assert_eq!(output.sealed_time, None);
        assert_eq!(output.request_id, None);
    }
}
