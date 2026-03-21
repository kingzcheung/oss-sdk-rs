//! GetBucketInfo 操作实现
//! 获取 Bucket 的详细信息

use std::sync::Arc;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::GetBucketInfoOutput;

/// GetBucketInfo Fluent Builder
#[derive(Debug)]
pub struct GetBucketInfoFluentBuilder {
    handle: Arc<Handle>,
    inner: GetBucketInfoInputBuilder,
}

#[derive(Debug, Default)]
struct GetBucketInfoInputBuilder {
    bucket: Option<String>,
}

impl GetBucketInfoFluentBuilder {
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

    /// 发送请求
    /// 
    /// # 返回
    /// 
    /// 返回 `GetBucketInfoOutput`，包含 Bucket 的详细信息
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
    /// let output = client.get_bucket_info()
    ///     .bucket("my-bucket")
    ///     .send()
    ///     .await?;
    /// 
    /// println!("Bucket name: {}", output.bucket.name);
    /// println!("Location: {}", output.bucket.location);
    /// println!("Storage class: {}", output.bucket.storage_class);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<GetBucketInfoOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or_else(|| {
            OSSError::Config("bucket is required for get_bucket_info".to_string())
        })?;

        // 构建查询参数
        // GET /?bucketInfo
        let query = "bucketInfo";

        // GetBucketInfo 请求需要指定 bucket
        let req = self.handle.build_request(
            HttpMethod::Get,
            "/",
            Some(&bucket),
            None,  // 不指定 object key
            Default::default(),
            Some(query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let text = resp.text().await?;
            let output: GetBucketInfoOutput = quick_xml::de::from_str(&text)
                .map_err(|e| OSSError::XmlParse(e.to_string()))?;
            Ok(output)
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