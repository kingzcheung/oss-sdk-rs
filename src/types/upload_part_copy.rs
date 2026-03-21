//! UploadPartCopy 操作类型定义
//!
//! 从一个已存在的 Object 中拷贝数据来上传一个 Part。

use serde::Deserialize;

/// UploadPartCopy 输入
#[derive(Debug, Clone, Default)]
pub struct UploadPartCopyInput {
    /// 目标 Bucket 名称
    pub bucket: String,
    /// 目标 Object 名称
    pub key: String,
    /// 分片上传 ID
    pub upload_id: String,
    /// 分片号，范围 1~10000
    pub part_number: u32,
    /// 源 Bucket 名称
    pub source_bucket: String,
    /// 源 Object 名称
    pub source_key: String,
    /// 源 Object 版本 ID
    pub source_version_id: Option<String>,
    /// 拷贝范围，格式：bytes=first-last
    pub copy_source_range: Option<String>,
    /// 如果源 Object 的 ETag 值和用户提供的 ETag 相等，则执行拷贝操作
    pub copy_source_if_match: Option<String>,
    /// 如果传入的 ETag 值和 Object 的 ETag 不匹配，则正常传输文件
    pub copy_source_if_none_match: Option<String>,
    /// 如果传入参数中的时间等于或者晚于文件实际修改时间，则正常传输文件
    pub copy_source_if_unmodified_since: Option<String>,
    /// 如果指定的时间早于实际修改时间，则正常传送文件
    pub copy_source_if_modified_since: Option<String>,
}

/// UploadPartCopy 输出
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "CopyPartResult")]
pub struct UploadPartCopyOutput {
    /// 最后修改时间
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    /// ETag
    #[serde(rename = "ETag")]
    pub etag: String,
}

/// UploadPartCopy 额外响应信息
#[derive(Debug, Clone, Default)]
pub struct UploadPartCopyResponse {
    /// 拷贝结果
    pub copy_part_result: Option<UploadPartCopyOutput>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// 源 Object 版本 ID
    pub copy_source_version_id: Option<String>,
}

/// UploadPartCopy 输入构建器
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct UploadPartCopyInputBuilder {
    inner: UploadPartCopyInput,
}

#[allow(dead_code)]
impl UploadPartCopyInputBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置目标 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// 设置目标 Object 名称
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

    /// 设置源 Bucket 名称
    pub fn source_bucket(mut self, source_bucket: impl Into<String>) -> Self {
        self.inner.source_bucket = source_bucket.into();
        self
    }

    /// 设置源 Object 名称
    pub fn source_key(mut self, source_key: impl Into<String>) -> Self {
        self.inner.source_key = source_key.into();
        self
    }

    /// 设置源 Object 版本 ID
    pub fn source_version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.source_version_id = Some(version_id.into());
        self
    }

    /// 设置拷贝范围，格式：bytes=first-last
    /// 例如：bytes=0-9 表示拷贝 0 到 9 这 10 个字节
    pub fn copy_source_range(mut self, range: impl Into<String>) -> Self {
        self.inner.copy_source_range = Some(range.into());
        self
    }

    /// 设置拷贝范围的起始和结束字节
    /// 自动生成 bytes=first-last 格式
    pub fn copy_source_range_bytes(mut self, first: u64, last: u64) -> Self {
        self.inner.copy_source_range = Some(format!("bytes={}-{}", first, last));
        self
    }

    /// 设置如果源 Object 的 ETag 值和用户提供的 ETag 相等，则执行拷贝操作
    pub fn copy_source_if_match(mut self, etag: impl Into<String>) -> Self {
        self.inner.copy_source_if_match = Some(etag.into());
        self
    }

    /// 设置如果传入的 ETag 值和 Object 的 ETag 不匹配，则正常传输文件
    pub fn copy_source_if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.inner.copy_source_if_none_match = Some(etag.into());
        self
    }

    /// 设置如果传入参数中的时间等于或者晚于文件实际修改时间，则正常传输文件
    pub fn copy_source_if_unmodified_since(mut self, time: impl Into<String>) -> Self {
        self.inner.copy_source_if_unmodified_since = Some(time.into());
        self
    }

    /// 设置如果指定的时间早于实际修改时间，则正常传送文件
    pub fn copy_source_if_modified_since(mut self, time: impl Into<String>) -> Self {
        self.inner.copy_source_if_modified_since = Some(time.into());
        self
    }

    /// 构建 Input
    pub fn build(self) -> Result<UploadPartCopyInput, &'static str> {
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
        if self.inner.source_bucket.is_empty() {
            return Err("source_bucket is required");
        }
        if self.inner.source_key.is_empty() {
            return Err("source_key is required");
        }
        Ok(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = UploadPartCopyInputBuilder::new()
            .bucket("dest-bucket")
            .key("dest-object.zip")
            .upload_id("0004B9895DBBB6EC9****")
            .part_number(1)
            .source_bucket("src-bucket")
            .source_key("src-object.zip")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "dest-bucket");
        assert_eq!(input.key, "dest-object.zip");
        assert_eq!(input.upload_id, "0004B9895DBBB6EC9****");
        assert_eq!(input.part_number, 1);
        assert_eq!(input.source_bucket, "src-bucket");
        assert_eq!(input.source_key, "src-object.zip");
    }

    #[test]
    fn test_input_builder_with_range() {
        let input = UploadPartCopyInputBuilder::new()
            .bucket("dest-bucket")
            .key("dest-object.zip")
            .upload_id("upload-id")
            .part_number(1)
            .source_bucket("src-bucket")
            .source_key("src-object.zip")
            .copy_source_range_bytes(100, 6291756)
            .build()
            .unwrap();

        assert_eq!(
            input.copy_source_range,
            Some("bytes=100-6291756".to_string())
        );
    }

    #[test]
    fn test_input_builder_with_version() {
        let input = UploadPartCopyInputBuilder::new()
            .bucket("dest-bucket")
            .key("dest-object.zip")
            .upload_id("upload-id")
            .part_number(1)
            .source_bucket("src-bucket")
            .source_key("src-object.zip")
            .source_version_id("version-123")
            .build()
            .unwrap();

        assert_eq!(input.source_version_id, Some("version-123".to_string()));
    }

    #[test]
    fn test_input_builder_with_conditions() {
        let input = UploadPartCopyInputBuilder::new()
            .bucket("dest-bucket")
            .key("dest-object.zip")
            .upload_id("upload-id")
            .part_number(1)
            .source_bucket("src-bucket")
            .source_key("src-object.zip")
            .copy_source_if_match("etag-123")
            .copy_source_if_unmodified_since("Fri, 13 Nov 2015 14:47:53 GMT")
            .build()
            .unwrap();

        assert_eq!(input.copy_source_if_match, Some("etag-123".to_string()));
        assert_eq!(
            input.copy_source_if_unmodified_since,
            Some("Fri, 13 Nov 2015 14:47:53 GMT".to_string())
        );
    }

    #[test]
    fn test_input_builder_missing_bucket() {
        let result = UploadPartCopyInputBuilder::new()
            .key("dest-object.zip")
            .upload_id("upload-id")
            .part_number(1)
            .source_bucket("src-bucket")
            .source_key("src-object.zip")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "bucket is required");
    }

    #[test]
    fn test_input_builder_missing_source() {
        let result = UploadPartCopyInputBuilder::new()
            .bucket("dest-bucket")
            .key("dest-object.zip")
            .upload_id("upload-id")
            .part_number(1)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "source_bucket is required");
    }

    #[test]
    fn test_deserialize_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CopyPartResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <LastModified>2014-07-17T06:27:54.000Z</LastModified>
    <ETag>"5B3C1A2E053D763E1B002CC607C5****"</ETag>
</CopyPartResult>"#;

        let output: UploadPartCopyOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.last_modified, "2014-07-17T06:27:54.000Z");
        assert_eq!(output.etag, "\"5B3C1A2E053D763E1B002CC607C5****\"");
    }
}
