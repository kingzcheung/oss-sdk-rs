//! SealAppendObject 操作实现
//! 停止对某个 Appendable Object 继续追加内容，并将其转为非追加状态

use std::sync::Arc;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::SealAppendObjectOutput;

/// SealAppendObject Fluent Builder
#[derive(Debug)]
pub struct SealAppendObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: SealAppendObjectInputBuilder,
}

#[derive(Debug, Default)]
struct SealAppendObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    position: Option<u64>,
}

impl SealAppendObjectFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = Some(bucket.into());
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 设置 Object 的预期长度
    /// OSS 会检查此长度与 Object 的实际长度是否一致
    /// 如果不一致，请求将失败并返回 PositionNotEqualToLength 错误
    pub fn position(mut self, position: u64) -> Self {
        self.inner.position = Some(position);
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `SealAppendObjectOutput`，包含封存时间等信息
    ///
    /// # 错误
    ///
    /// 如果请求失败，返回 `OSSError`
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use oss_sdk_rs::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // 先追加内容
    /// let append_output = client.append_object()
    ///     .bucket("my-bucket")
    ///     .key("append-file.txt")
    ///     .position(0)
    ///     .body(b"Hello World".to_vec())
    ///     .send()
    ///     .await?;
    ///
    /// // 然后封存
    /// let seal_output = client.seal_append_object()
    ///     .bucket("my-bucket")
    ///     .key("append-file.txt")
    ///     .position(append_output.next_append_position.unwrap())
    ///     .send()
    ///     .await?;
    ///
    /// println!("Sealed time: {:?}", seal_output.sealed_time);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<SealAppendObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or_else(|| {
            OSSError::Config("bucket is required for seal_append_object".to_string())
        })?;
        let key = self.inner.key.ok_or_else(|| {
            OSSError::Config("key is required for seal_append_object".to_string())
        })?;
        let position = self.inner.position.ok_or_else(|| {
            OSSError::Config("position is required for seal_append_object".to_string())
        })?;

        // 构建查询参数
        // POST /ObjectName?seal&position=Position
        let query = format!("seal&position={}", position);

        // SealAppendObject 使用 POST 方法，Content-Length: 0
        let req = self.handle.build_request(
            HttpMethod::Post,
            &format!("/{}", key),
            Some(&bucket),
            Some(&key),
            Default::default(),
            Some(&query),
        )?;

        let req = req.body(Vec::new());
        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let headers = resp.headers().clone();

            let etag = headers
                .get("ETag")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim_matches('"').to_string());

            let object_type = headers
                .get("x-oss-object-type")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let storage_class = headers
                .get("x-oss-storage-class")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let sealed_time = headers
                .get("x-oss-sealed-time")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let last_modified = headers
                .get("Last-Modified")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let content_length = headers
                .get("Content-Length")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            let content_type = headers
                .get("Content-Type")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let request_id = headers
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(SealAppendObjectOutput {
                etag,
                object_type,
                storage_class,
                sealed_time,
                last_modified,
                content_length,
                content_type,
                request_id,
            })
        } else {
            let text = resp.text().await?;
            Err(OSSError::Object {
                status_code: status,
                message: text,
                raw_response: serde_json::Value::Null,
            })
        }
    }
}
