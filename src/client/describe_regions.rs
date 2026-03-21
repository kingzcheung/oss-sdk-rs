//! DescribeRegions 操作实现
//! 查询所有支持地域或指定地域对应的 Endpoint 信息

use std::sync::Arc;

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::DescribeRegionsOutput;

/// DescribeRegions Fluent Builder
#[derive(Debug)]
pub struct DescribeRegionsFluentBuilder {
    handle: Arc<Handle>,
    inner: DescribeRegionsInputBuilder,
}

#[derive(Debug, Default)]
struct DescribeRegionsInputBuilder {
    region: Option<String>,
}

impl DescribeRegionsFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置 OSS 专用 Region ID
    /// 如果不指定，则返回所有支持地域对应的 Endpoint 信息
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.inner.region = Some(region.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `DescribeRegionsOutput`，包含地域信息列表
    ///
    /// # 错误
    ///
    /// 如果请求失败，返回 `OSSError`
    pub async fn send(self) -> Result<DescribeRegionsOutput, OSSError> {
        // 构建查询参数
        // GET /?regions 或 GET /?regions=oss-cn-hangzhou
        let query = if let Some(region) = &self.inner.region {
            format!("regions={}", urlencoding::encode(region))
        } else {
            "regions".to_string()
        };

        // DescribeRegions 请求不需要 bucket 参数
        let req = self.handle.build_request(
            HttpMethod::Get,
            "/",
            None, // 不指定 bucket
            None, // 不指定 object key
            Default::default(),
            Some(&query),
        )?;

        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let text = resp.text().await?;
            let output: DescribeRegionsOutput =
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
