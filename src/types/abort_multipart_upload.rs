//! AbortMultipartUpload 接口的类型定义
//!
//! 用于取消 MultipartUpload 事件并删除对应的 Part 数据

use serde::Deserialize;

/// AbortMultipartUpload 操作的输入
#[derive(Debug, Clone)]
pub struct AbortMultipartUploadInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// MultipartUpload 事件的唯一标识
    pub upload_id: String,
}

impl AbortMultipartUploadInput {
    /// 创建新的输入
    pub fn new(bucket: impl Into<String>, key: impl Into<String>, upload_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            upload_id: upload_id.into(),
        }
    }
}

/// AbortMultipartUpload 操作的输出
#[derive(Debug, Clone)]
pub struct AbortMultipartUploadOutput {
    /// 请求 ID
    pub request_id: String,
    /// ETag 值（如果返回）
    pub etag: Option<String>,
}

impl AbortMultipartUploadOutput {
    /// 创建新的输出
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            etag: None,
        }
    }

    /// 设置 ETag
    pub fn with_etag(mut self, etag: impl Into<String>) -> Self {
        self.etag = Some(etag.into());
        self
    }
}

/// 错误响应
#[derive(Debug, Clone, Deserialize)]
pub struct AbortMultipartUploadError {
    /// 错误码
    #[serde(rename = "Code")]
    pub code: String,
    /// 错误信息
    #[serde(rename = "Message")]
    pub message: String,
    /// 请求 ID
    #[serde(rename = "RequestId")]
    pub request_id: String,
    /// Host ID
    #[serde(rename = "HostId")]
    pub host_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abort_multipart_upload_input() {
        let input = AbortMultipartUploadInput::new("my-bucket", "large-file.zip", "upload-id-123");
        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "large-file.zip");
        assert_eq!(input.upload_id, "upload-id-123");
    }

    #[test]
    fn test_abort_multipart_upload_output() {
        let output = AbortMultipartUploadOutput::new("request-id-123");
        assert_eq!(output.request_id, "request-id-123");
        assert!(output.etag.is_none());

        let output_with_etag = output.with_etag("\"etag-abc\"");
        assert_eq!(output_with_etag.etag, Some("\"etag-abc\"".to_string()));
    }
}