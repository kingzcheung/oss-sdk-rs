//! UploadPart 操作类型定义
//!
//! 根据指定的 Object 名和 uploadId 来分片上传数据。

/// UploadPart 输入
#[derive(Debug, Clone, Default)]
pub struct UploadPartInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// 分片上传 ID
    pub upload_id: String,
    /// 分片号，范围 1~10000
    pub part_number: u32,
    /// 分片数据
    pub body: Vec<u8>,
}

/// UploadPart 输出
#[derive(Debug, Clone)]
pub struct UploadPartOutput {
    /// ETag，分片的 MD5 值
    pub etag: Option<String>,
    /// 分片的 MD5 值
    pub content_md5: Option<String>,
    /// 分片的 CRC64 值
    pub crc64_ecma: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// 服务端加密算法
    pub server_side_encryption: Option<String>,
}

impl Default for UploadPartOutput {
    fn default() -> Self {
        Self {
            etag: None,
            content_md5: None,
            crc64_ecma: None,
            request_id: None,
            server_side_encryption: None,
        }
    }
}

/// UploadPart 输入构建器
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct UploadPartInputBuilder {
    inner: UploadPartInput,
}

#[allow(dead_code)]
impl UploadPartInputBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = key.into();
        self
    }

    /// 设置分片上传 ID
    pub fn upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.inner.upload_id = upload_id.into();
        self
    }

    /// 设置分片号，范围 1~10000
    pub fn part_number(mut self, part_number: u32) -> Self {
        self.inner.part_number = part_number;
        self
    }

    /// 设置分片数据
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.inner.body = body.into();
        self
    }

    /// 构建 Input
    pub fn build(self) -> Result<UploadPartInput, &'static str> {
        if self.inner.bucket.is_empty() {
            return Err("bucket is required");
        }
        if self.inner.key.is_empty() {
            return Err("key is required");
        }
        if self.inner.upload_id.is_empty() {
            return Err("upload_id is required");
        }
        if self.inner.part_number < 1 || self.inner.part_number > 10000 {
            return Err("part_number must be between 1 and 10000");
        }
        Ok(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = UploadPartInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("0004B9895DBBB6EC9****")
            .part_number(1)
            .body(b"test data".to_vec())
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "large-file.zip");
        assert_eq!(input.upload_id, "0004B9895DBBB6EC9****");
        assert_eq!(input.part_number, 1);
        assert_eq!(input.body, b"test data".to_vec());
    }

    #[test]
    fn test_input_builder_missing_bucket() {
        let result = UploadPartInputBuilder::new()
            .key("large-file.zip")
            .upload_id("upload-id")
            .part_number(1)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "bucket is required");
    }

    #[test]
    fn test_input_builder_missing_key() {
        let result = UploadPartInputBuilder::new()
            .bucket("my-bucket")
            .upload_id("upload-id")
            .part_number(1)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "key is required");
    }

    #[test]
    fn test_input_builder_missing_upload_id() {
        let result = UploadPartInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .part_number(1)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "upload_id is required");
    }

    #[test]
    fn test_input_builder_invalid_part_number_zero() {
        let result = UploadPartInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .part_number(0)
            .build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "part_number must be between 1 and 10000"
        );
    }

    #[test]
    fn test_input_builder_invalid_part_number_too_large() {
        let result = UploadPartInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .part_number(10001)
            .build();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "part_number must be between 1 and 10000"
        );
    }

    #[test]
    fn test_output_default() {
        let output = UploadPartOutput::default();
        assert!(output.etag.is_none());
        assert!(output.content_md5.is_none());
        assert!(output.crc64_ecma.is_none());
        assert!(output.request_id.is_none());
        assert!(output.server_side_encryption.is_none());
    }
}
