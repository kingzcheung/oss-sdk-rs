//! DeleteMultipleObjects 操作实现
//! 删除同一个存储空间（Bucket）中的多个文件（Object）

use std::sync::Arc;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{Handle, HttpMethod};
use crate::errors::OSSError;
use crate::types::{
    to_xml, DeleteMultipleObjectsInput, DeleteMultipleObjectsOutput, ObjectIdentifier,
};

/// DeleteMultipleObjects Fluent Builder
#[derive(Debug)]
pub struct DeleteMultipleObjectsFluentBuilder {
    handle: Arc<Handle>,
    inner: DeleteMultipleObjectsInputBuilder,
}

#[derive(Debug, Default)]
struct DeleteMultipleObjectsInputBuilder {
    bucket: Option<String>,
    objects: Vec<ObjectIdentifier>,
    quiet: bool,
    encoding_type: Option<String>,
}

impl DeleteMultipleObjectsFluentBuilder {
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

    /// 添加要删除的 Object
    pub fn object(mut self, key: impl Into<String>) -> Self {
        self.inner.objects.push(ObjectIdentifier {
            key: key.into(),
            version_id: None,
        });
        self
    }

    /// 添加要删除的 Object（带版本 ID）
    pub fn object_with_version(
        mut self,
        key: impl Into<String>,
        version_id: impl Into<String>,
    ) -> Self {
        self.inner.objects.push(ObjectIdentifier {
            key: key.into(),
            version_id: Some(version_id.into()),
        });
        self
    }

    /// 设置要删除的 Object 列表
    pub fn objects(mut self, objects: Vec<ObjectIdentifier>) -> Self {
        self.inner.objects = objects;
        self
    }

    /// 设置是否开启简单响应模式
    ///
    /// - `true`: OSS 不返回消息体
    /// - `false`: OSS 返回消息体中包含所有删除 Object 的结果（默认）
    pub fn quiet(mut self, quiet: bool) -> Self {
        self.inner.quiet = quiet;
        self
    }

    /// 设置编码类型
    ///
    /// 如果 Key 中包含 XML 1.0 标准不支持的控制字符，可指定 Encoding-type 为 url2
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `DeleteMultipleObjectsOutput`，包含成功删除的 Object 列表
    ///
    /// # 错误
    ///
    /// 如果请求失败，返回 `OSSError`
    ///
    /// # 限制
    ///
    /// 单次请求最多允许删除 1000 个文件
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use oss_sdk_rs::types::ObjectIdentifier;
    ///
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// // 删除多个文件
    /// let output = client.delete_multiple_objects()
    ///     .bucket("my-bucket")
    ///     .object("file1.txt")
    ///     .object("file2.txt")
    ///     .object("file3.txt")
    ///     .send()
    ///     .await?;
    ///
    /// for deleted in &output.deleted {
    ///     println!("Deleted: {}", deleted.key);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<DeleteMultipleObjectsOutput, OSSError> {
        // 验证参数
        let bucket = self
            .inner
            .bucket
            .ok_or_else(|| OSSError::InvalidInput("bucket is required".to_string()))?;

        // 最多允许删除 1000 个文件
        if self.inner.objects.len() > 1000 {
            return Err(OSSError::InvalidInput(
                "maximum 1000 objects allowed per request".to_string(),
            ));
        }

        if self.inner.objects.is_empty() {
            return Err(OSSError::InvalidInput(
                "at least one object is required".to_string(),
            ));
        }

        // 构建输入
        let input = DeleteMultipleObjectsInput {
            bucket,
            objects: self.inner.objects,
            quiet: self.inner.quiet,
            encoding_type: self.inner.encoding_type,
        };

        // 构建 XML 请求体
        let xml_body = to_xml(&input).map_err(|e| OSSError::XmlParse(e.to_string()))?;

        // 计算 Content-MD5
        let md5_hash = md5::compute(xml_body.as_bytes());
        let content_md5 =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, md5_hash.0);

        // 构建请求头（必须在签名之前添加）
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/xml"));
        headers.insert(
            "Content-MD5",
            HeaderValue::from_str(&content_md5).map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );
        headers.insert(
            "Content-Length",
            HeaderValue::from_str(&xml_body.len().to_string())
                .map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );

        // 添加 Encoding-Type 请求头（如果指定）
        if let Some(ref encoding_type) = input.encoding_type {
            headers.insert(
                "Encoding-type",
                HeaderValue::from_str(encoding_type)
                    .map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }

        // 构建请求（签名会包含 Content-MD5 和 Content-Type 头部）
        let req = self.handle.build_request(
            HttpMethod::Post,
            "/",
            Some(&input.bucket),
            None,
            headers,
            Some("delete"),
        )?;

        // 设置请求体并发送请求
        let resp = req.body(xml_body).send().await?;

        let status = resp.status();

        // 获取请求 ID
        let request_id = resp
            .headers()
            .get("x-oss-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        if status.is_success() {
            // 如果是 quiet 模式，响应体可能为空
            let text = resp.text().await?;

            if text.is_empty() {
                // Quiet 模式，返回空结果
                return Ok(DeleteMultipleObjectsOutput {
                    deleted: vec![],
                    encoding_type: None,
                    request_id,
                });
            }

            // 解析 XML 响应
            let mut output: DeleteMultipleObjectsOutput =
                quick_xml::de::from_str(&text).map_err(|e| OSSError::XmlParse(e.to_string()))?;
            output.request_id = request_id;
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
