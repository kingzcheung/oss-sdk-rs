//! PostObject 操作实现
//!
//! 通过 HTML 表单上传的方式将文件（Object）上传到指定存储空间（Bucket）。
//!
//! # 示例
//!
//! ```no_run
//! use oss_sdk_rs::{Client, Config, Credentials};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new("your-access-key-id", "your-access-key-secret");
//!     let config = Config::builder()
//!         .credentials(credentials)
//!         .region("oss-cn-hangzhou")
//!         .build()?;
//!     let client = Client::from_config(config)?;
//!     
//!     let output = client.post_object()
//!         .bucket("my-bucket")
//!         .key("my-object.txt")
//!         .body(b"Hello, World!".to_vec())
//!         .send()
//!         .await?;
//!     
//!     println!("ETag: {:?}", output.etag);
//!     println!("Location: {:?}", output.location);
//!     
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use sha1::Sha1;

use crate::client::Handle;
use crate::errors::OSSError;
use crate::types::{PostObjectOutput, SuccessActionStatus};

type HmacSha1 = Hmac<Sha1>;

/// PostObject Fluent Builder
#[derive(Debug)]
pub struct PostObjectFluentBuilder {
    handle: Arc<Handle>,
    inner: PostObjectInputBuilder,
}

#[derive(Debug, Default)]
struct PostObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    body: Option<Vec<u8>>,
    policy: Option<String>,
    signature: Option<String>,
    oss_access_key_id: Option<String>,
    x_oss_signature: Option<String>,
    x_oss_date: Option<String>,
    x_oss_credential: Option<String>,
    x_oss_security_token: Option<String>,
    success_action_redirect: Option<String>,
    success_action_status: Option<SuccessActionStatus>,
    content_disposition: Option<String>,
    content_type: Option<String>,
    cache_control: Option<String>,
    content_encoding: Option<String>,
    expires: Option<String>,
    x_oss_object_acl: Option<String>,
    x_oss_storage_class: Option<String>,
    x_oss_server_side_encryption: Option<String>,
    x_oss_server_side_encryption_key_id: Option<String>,
    x_oss_server_side_data_encryption: Option<String>,
    x_oss_forbid_overwrite: Option<bool>,
    user_metadata: std::collections::HashMap<String, String>,
}

impl PostObjectFluentBuilder {
    pub(crate) fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// 设置存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = Some(bucket.into());
        self
    }

    /// 设置对象键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = Some(key.into());
        self
    }

    /// 设置文件内容
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.inner.body = Some(body.into());
        self
    }

    /// 设置 Policy（Base64 编码）
    /// 如果不设置，将自动生成
    pub fn policy(mut self, policy: impl Into<String>) -> Self {
        self.inner.policy = Some(policy.into());
        self
    }

    /// 设置 V1 签名
    /// 如果不设置，将自动生成
    pub fn signature(mut self, signature: impl Into<String>) -> Self {
        self.inner.signature = Some(signature.into());
        self
    }

    /// 设置 OSSAccessKeyId
    /// 如果不设置，将自动使用配置中的凭证
    pub fn oss_access_key_id(mut self, oss_access_key_id: impl Into<String>) -> Self {
        self.inner.oss_access_key_id = Some(oss_access_key_id.into());
        self
    }

    /// 设置 V4 签名
    pub fn x_oss_signature(mut self, x_oss_signature: impl Into<String>) -> Self {
        self.inner.x_oss_signature = Some(x_oss_signature.into());
        self
    }

    /// 设置 V4 日期
    pub fn x_oss_date(mut self, x_oss_date: impl Into<String>) -> Self {
        self.inner.x_oss_date = Some(x_oss_date.into());
        self
    }

    /// 设置 V4 Credential
    pub fn x_oss_credential(mut self, x_oss_credential: impl Into<String>) -> Self {
        self.inner.x_oss_credential = Some(x_oss_credential.into());
        self
    }

    /// 设置安全令牌
    pub fn x_oss_security_token(mut self, x_oss_security_token: impl Into<String>) -> Self {
        self.inner.x_oss_security_token = Some(x_oss_security_token.into());
        self
    }

    /// 设置成功后跳转 URL
    pub fn success_action_redirect(mut self, success_action_redirect: impl Into<String>) -> Self {
        self.inner.success_action_redirect = Some(success_action_redirect.into());
        self
    }

    /// 设置成功后返回状态码
    pub fn success_action_status(mut self, success_action_status: SuccessActionStatus) -> Self {
        self.inner.success_action_status = Some(success_action_status);
        self
    }

    /// 设置 Content-Disposition
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.inner.content_disposition = Some(content_disposition.into());
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// 设置 Cache-Control
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.inner.cache_control = Some(cache_control.into());
        self
    }

    /// 设置 Content-Encoding
    pub fn content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.inner.content_encoding = Some(content_encoding.into());
        self
    }

    /// 设置 Expires
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.inner.expires = Some(expires.into());
        self
    }

    /// 设置 Object 访问权限
    pub fn x_oss_object_acl(mut self, x_oss_object_acl: impl Into<String>) -> Self {
        self.inner.x_oss_object_acl = Some(x_oss_object_acl.into());
        self
    }

    /// 设置存储类型
    pub fn x_oss_storage_class(mut self, x_oss_storage_class: impl Into<String>) -> Self {
        self.inner.x_oss_storage_class = Some(x_oss_storage_class.into());
        self
    }

    /// 设置服务端加密算法
    pub fn x_oss_server_side_encryption(mut self, x_oss_server_side_encryption: impl Into<String>) -> Self {
        self.inner.x_oss_server_side_encryption = Some(x_oss_server_side_encryption.into());
        self
    }

    /// 设置 KMS 密钥 ID
    pub fn x_oss_server_side_encryption_key_id(mut self, x_oss_server_side_encryption_key_id: impl Into<String>) -> Self {
        self.inner.x_oss_server_side_encryption_key_id = Some(x_oss_server_side_encryption_key_id.into());
        self
    }

    /// 设置服务端数据加密算法
    pub fn x_oss_server_side_data_encryption(mut self, x_oss_server_side_data_encryption: impl Into<String>) -> Self {
        self.inner.x_oss_server_side_data_encryption = Some(x_oss_server_side_data_encryption.into());
        self
    }

    /// 设置是否禁止覆盖同名 Object
    pub fn x_oss_forbid_overwrite(mut self, x_oss_forbid_overwrite: bool) -> Self {
        self.inner.x_oss_forbid_overwrite = Some(x_oss_forbid_overwrite);
        self
    }

    /// 添加用户自定义元数据
    pub fn user_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.user_metadata.insert(key.into(), value.into());
        self
    }

    /// 发送请求
    ///
    /// # 返回
    ///
    /// 返回 `PostObjectOutput`，包含上传结果
    ///
    /// # 错误
    ///
    /// - 如果上传失败，返回相应的错误
    ///
    /// # 示例
    ///
    /// ```no_run
    /// # async fn example(client: oss_sdk_rs::Client) -> Result<(), oss_sdk_rs::errors::OSSError> {
    /// let output = client.post_object()
    ///     .bucket("my-bucket")
    ///     .key("my-object.txt")
    ///     .body(b"Hello, World!".to_vec())
    ///     .send()
    ///     .await?;
    ///
    /// println!("ETag: {:?}", output.etag);
    /// println!("Location: {:?}", output.location);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> Result<PostObjectOutput, OSSError> {
        let bucket = self.inner.bucket.ok_or(OSSError::BucketNotSet)?;
        let key = self.inner.key.ok_or(OSSError::KeyNotSet)?;
        let body = self.inner.body.ok_or_else(|| OSSError::InvalidInput("body is required".to_string()))?;

        // 获取凭证
        let credentials = self.handle.config.credentials()
            .ok_or_else(|| OSSError::Credentials("credentials not set".to_string()))?;
        
        let access_key_id = self.inner.oss_access_key_id.clone()
            .unwrap_or_else(|| credentials.access_key_id().to_string());
        let access_key_secret = credentials.access_key_secret();

        // 生成 policy（如果没有提供）
        let (policy, signature) = if let Some(ref p) = self.inner.policy {
            // 使用提供的 policy，需要计算签名
            let sig = self.inner.signature.clone().unwrap_or_else(|| {
                calculate_signature(access_key_secret, p)
            });
            (p.clone(), sig)
        } else {
            // 自动生成 policy 和签名
            let expiration = generate_expiration(3600); // 1 小时后过期
            let policy_json = build_policy(&bucket, &key, &expiration, body.len());
            let policy_base64 = STANDARD.encode(&policy_json);
            let sig = calculate_signature(access_key_secret, &policy_base64);
            (policy_base64, sig)
        };

        // 生成 boundary
        let boundary = generate_boundary();

        // 构建 multipart/form-data 消息体
        let mut form_data = Vec::new();

        // 添加 key 表单域
        form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"key\"\r\n\r\n");
        form_data.extend_from_slice(key.as_bytes());
        form_data.extend_from_slice(b"\r\n");

        // 添加 OSSAccessKeyId
        form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"OSSAccessKeyId\"\r\n\r\n");
        form_data.extend_from_slice(access_key_id.as_bytes());
        form_data.extend_from_slice(b"\r\n");

        // 添加 policy
        form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"policy\"\r\n\r\n");
        form_data.extend_from_slice(policy.as_bytes());
        form_data.extend_from_slice(b"\r\n");

        // 添加 Signature
        form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"Signature\"\r\n\r\n");
        form_data.extend_from_slice(signature.as_bytes());
        form_data.extend_from_slice(b"\r\n");

        // 添加可选表单域
        if let Some(ref x_oss_security_token) = self.inner.x_oss_security_token {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"x-oss-security-token\"\r\n\r\n");
            form_data.extend_from_slice(x_oss_security_token.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref success_action_redirect) = self.inner.success_action_redirect {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"success_action_redirect\"\r\n\r\n");
            form_data.extend_from_slice(success_action_redirect.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref success_action_status) = self.inner.success_action_status {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"success_action_status\"\r\n\r\n");
            form_data.extend_from_slice(success_action_status.to_string().as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        // 添加 HTTP 头部相关表单域
        if let Some(ref content_disposition) = self.inner.content_disposition {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"Content-Disposition\"\r\n\r\n");
            form_data.extend_from_slice(content_disposition.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref cache_control) = self.inner.cache_control {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"Cache-Control\"\r\n\r\n");
            form_data.extend_from_slice(cache_control.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref content_encoding) = self.inner.content_encoding {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"Content-Encoding\"\r\n\r\n");
            form_data.extend_from_slice(content_encoding.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref expires) = self.inner.expires {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"Expires\"\r\n\r\n");
            form_data.extend_from_slice(expires.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        // 添加 OSS 扩展字段
        if let Some(ref x_oss_object_acl) = self.inner.x_oss_object_acl {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"x-oss-object-acl\"\r\n\r\n");
            form_data.extend_from_slice(x_oss_object_acl.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref x_oss_storage_class) = self.inner.x_oss_storage_class {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"x-oss-storage-class\"\r\n\r\n");
            form_data.extend_from_slice(x_oss_storage_class.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref x_oss_server_side_encryption) = self.inner.x_oss_server_side_encryption {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"x-oss-server-side-encryption\"\r\n\r\n");
            form_data.extend_from_slice(x_oss_server_side_encryption.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref x_oss_server_side_encryption_key_id) = self.inner.x_oss_server_side_encryption_key_id {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"x-oss-server-side-encryption-key-id\"\r\n\r\n");
            form_data.extend_from_slice(x_oss_server_side_encryption_key_id.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(ref x_oss_server_side_data_encryption) = self.inner.x_oss_server_side_data_encryption {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"x-oss-server-side-data-encryption\"\r\n\r\n");
            form_data.extend_from_slice(x_oss_server_side_data_encryption.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        if let Some(x_oss_forbid_overwrite) = self.inner.x_oss_forbid_overwrite {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(b"Content-Disposition: form-data; name=\"x-oss-forbid-overwrite\"\r\n\r\n");
            form_data.extend_from_slice(if x_oss_forbid_overwrite { b"true" } else { b"false" });
            form_data.extend_from_slice(b"\r\n");
        }

        // 添加用户自定义元数据
        for (meta_key, meta_value) in &self.inner.user_metadata {
            form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            form_data.extend_from_slice(format!("Content-Disposition: form-data; name=\"x-oss-meta-{}\"\r\n\r\n", meta_key).as_bytes());
            form_data.extend_from_slice(meta_value.as_bytes());
            form_data.extend_from_slice(b"\r\n");
        }

        // 添加 file 表单域（必须是最后一个）
        let content_type = self.inner.content_type.clone().unwrap_or_else(|| "application/octet-stream".to_string());
        form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        form_data.extend_from_slice(format!("Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n", key).as_bytes());
        form_data.extend_from_slice(format!("Content-Type: {}\r\n\r\n", content_type).as_bytes());
        form_data.extend_from_slice(&body);
        form_data.extend_from_slice(b"\r\n");

        // 结束边界
        form_data.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        // 构建请求 URL
        let endpoint = self.handle.config.endpoint().unwrap_or("oss-cn-hangzhou.aliyuncs.com");
        // 移除 endpoint 中的协议前缀
        let endpoint_host = endpoint
            .strip_prefix("https://")
            .or_else(|| endpoint.strip_prefix("http://"))
            .unwrap_or(endpoint);
        
        let url = format!("https://{}.{}", bucket, endpoint_host);

        // 构建请求头
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            format!("multipart/form-data; boundary={}", boundary)
                .parse()
                .map_err(|e| OSSError::InvalidHeaderValue(e))?,
        );

        // 发送请求
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .headers(headers)
            .body(form_data)
            .send()
            .await?;

        let status = response.status();
        let resp_headers = response.headers().clone();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(OSSError::Object {
                status_code: status,
                message: error_text,
                raw_response: serde_json::Value::Null,
            });
        }

        // 解析响应头
        let output = PostObjectOutput {
            etag: resp_headers
                .get("ETag")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.trim_matches('"').to_string()),
            location: resp_headers
                .get("Location")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            bucket: Some(bucket),
            key: Some(key),
            version_id: resp_headers
                .get("x-oss-version-id")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            request_id: resp_headers
                .get("x-oss-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            hash_crc64ecma: resp_headers
                .get("x-oss-hash-crc64ecma")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            content_md5: resp_headers
                .get("Content-MD5")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
            server_side_encryption: resp_headers
                .get("x-oss-server-side-encryption")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_string()),
        };

        Ok(output)
    }
}

/// 生成过期时间字符串（ISO 8601 格式）
fn generate_expiration(seconds: u64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let expiry = now + seconds;
    
    // 转换为 UTC 时间
    let days = expiry / 86400;
    let secs = expiry % 86400;
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    
    // 计算年份和月份（简化版本）
    let year = 1970 + days / 365;
    let day_of_year = days % 365;
    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;
    
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hours, minutes, seconds)
}

/// 构建 policy JSON
fn build_policy(bucket: &str, key: &str, expiration: &str, content_length: usize) -> String {
    format!(
        r#"{{"expiration":"{}","conditions":[{{"bucket":"{}"}},["starts-with","$key","{}"],["content-length-range",0,{}]]}}"#,
        expiration, bucket, key, content_length
    )
}

/// 计算签名
fn calculate_signature(access_key_secret: &str, policy: &str) -> String {
    let mut mac = HmacSha1::new_from_slice(access_key_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(policy.as_bytes());
    let result = mac.finalize();
    STANDARD.encode(result.into_bytes())
}

/// 生成随机 boundary 字符串
fn generate_boundary() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("----OSSBoundary{}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_bucket_key_body() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.post_object()
            .bucket("my-bucket")
            .key("my-object.txt")
            .body(b"test content".to_vec());

        assert_eq!(builder.inner.bucket, Some("my-bucket".to_string()));
        assert_eq!(builder.inner.key, Some("my-object.txt".to_string()));
        assert_eq!(builder.inner.body, Some(b"test content".to_vec()));
    }

    #[test]
    fn test_builder_with_options() {
        let config = crate::config::Config::builder()
            .credentials(crate::credentials::Credentials::new("test", "test"))
            .region("oss-cn-hangzhou")
            .build()
            .unwrap();

        let client = super::super::Client::from_config(config).unwrap();

        let builder = client.post_object()
            .bucket("my-bucket")
            .key("my-object.txt")
            .body(b"test content".to_vec())
            .content_type("text/plain")
            .success_action_status(SuccessActionStatus::Status200)
            .user_metadata("uuid", "my-uuid");

        assert_eq!(builder.inner.content_type, Some("text/plain".to_string()));
        assert_eq!(builder.inner.success_action_status, Some(SuccessActionStatus::Status200));
        assert_eq!(builder.inner.user_metadata.get("uuid"), Some(&"my-uuid".to_string()));
    }

    #[test]
    fn test_generate_boundary() {
        let boundary1 = generate_boundary();
        let boundary2 = generate_boundary();
        
        // 每次生成的 boundary 应该不同
        assert_ne!(boundary1, boundary2);
        
        // boundary 应该以 ----OSSBoundary 开头
        assert!(boundary1.starts_with("----OSSBoundary"));
        assert!(boundary2.starts_with("----OSSBoundary"));
    }

    #[test]
    fn test_calculate_signature() {
        let secret = "test-secret";
        let policy = "test-policy";
        let sig = calculate_signature(secret, policy);
        
        // 签名应该是 Base64 编码的字符串
        assert!(!sig.is_empty());
        assert!(STANDARD.decode(&sig).is_ok());
    }

    #[test]
    fn test_build_policy() {
        let policy = build_policy("my-bucket", "my-key", "2024-01-01T00:00:00Z", 1000);
        
        // 验证 policy 是有效的 JSON
        assert!(serde_json::from_str::<serde_json::Value>(&policy).is_ok());
        assert!(policy.contains("my-bucket"));
        assert!(policy.contains("my-key"));
        assert!(policy.contains("1000"));
    }
}