//! GetBucketStat 操作实现
//! 获取指定 Bucket 的存储容量、文件以及 Multipart 分片数量

use std::sync::Arc;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::GetBucketStatOutput;

/// GetBucketStat Fluent Builder
#[derive(Debug)]
pub struct GetBucketStatFluentBuilder {
    handle: Arc<Handle>,
    inner: GetBucketStatInputBuilder,
}

#[derive(Debug, Default)]
struct GetBucketStatInputBuilder {
    bucket: Option<String>,
}

impl GetBucketStatFluentBuilder {
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
    /// 返回 `GetBucketStatOutput`，包含 Bucket 的存储统计信息
    ///
    /// # 错误
    ///
    /// 如果请求失败，返回 `OSSError`
    ///
    /// # 注意事项
    ///
    /// - 调用该接口获取的数据并非是实时数据，延时可能超过一个小时
    /// - 调用该接口获取到的存储信息的时间点不保证是最新的
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use oss_sdk_rs::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let output = client.get_bucket_stat()
    ///     .bucket("my-bucket")
    ///     .send()
    ///     .await?;
    ///
    /// if let Some(storage) = output.storage {
    ///     println!("Storage: {} bytes", storage);
    /// }
    /// if let Some(count) = output.object_count {
    ///     println!("Object count: {}", count);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<GetBucketStatOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or_else(|| {
            OSSError::Config("bucket is required for get_bucket_stat".to_string())
        })?;

        // 构建查询参数
        // GET /?stat
        let query = "stat";

        // GetBucketStat 请求需要指定 bucket
        let req = self.handle.build_request(
            HttpMethod::Get,
            "/",
            Some(&bucket),
            None, // 不指定 object key
            Default::default(),
            Some(query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let text = resp.text().await?;
            let output: GetBucketStatOutput =
                quick_xml::de::from_str(&text).map_err(|e| OSSError::XmlParse(e.to_string()))?;
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
