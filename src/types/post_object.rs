//! PostObject Input/Output 类型定义
//!
//! 通过 HTML 表单上传的方式将文件（Object）上传到指定存储空间（Bucket）。

use std::collections::HashMap;

/// PostObject 操作输入
#[derive(Debug, Clone)]
pub struct PostObjectInput {
    /// 存储桶名称
    pub bucket: String,
    /// 上传 Object 的名称
    pub key: String,
    /// 文件内容
    pub body: Vec<u8>,
    /// Policy 规则（Base64 编码的 JSON）
    pub policy: Option<String>,
    /// V1 签名
    pub signature: Option<String>,
    /// OSSAccessKeyId
    pub oss_access_key_id: Option<String>,
    /// V4 签名相关字段
    pub x_oss_signature: Option<String>,
    pub x_oss_date: Option<String>,
    pub x_oss_credential: Option<String>,
    /// 安全令牌（STS）
    pub x_oss_security_token: Option<String>,
    /// 成功后跳转 URL
    pub success_action_redirect: Option<String>,
    /// 成功后返回状态码
    pub success_action_status: Option<SuccessActionStatus>,
    /// Content-Disposition
    pub content_disposition: Option<String>,
    /// Content-Type
    pub content_type: Option<String>,
    /// Cache-Control
    pub cache_control: Option<String>,
    /// Content-Encoding
    pub content_encoding: Option<String>,
    /// Expires
    pub expires: Option<String>,
    /// Object 访问权限
    pub x_oss_object_acl: Option<String>,
    /// 存储类型
    pub x_oss_storage_class: Option<String>,
    /// 服务端加密算法
    pub x_oss_server_side_encryption: Option<String>,
    /// KMS 密钥 ID
    pub x_oss_server_side_encryption_key_id: Option<String>,
    /// 服务端数据加密算法
    pub x_oss_server_side_data_encryption: Option<String>,
    /// 是否禁止覆盖同名 Object
    pub x_oss_forbid_overwrite: Option<bool>,
    /// 用户自定义元数据
    pub user_metadata: HashMap<String, String>,
}

/// 成功后返回状态码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuccessActionStatus {
    /// 返回空文档和 200 状态码
    Status200,
    /// 返回 XML 文档和 201 状态码
    Status201,
    /// 返回空文档和 204 状态码（默认）
    Status204,
}

impl Default for SuccessActionStatus {
    fn default() -> Self {
        Self::Status204
    }
}

impl std::fmt::Display for SuccessActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuccessActionStatus::Status200 => write!(f, "200"),
            SuccessActionStatus::Status201 => write!(f, "201"),
            SuccessActionStatus::Status204 => write!(f, "204"),
        }
    }
}

impl From<&str> for SuccessActionStatus {
    fn from(s: &str) -> Self {
        match s {
            "200" => SuccessActionStatus::Status200,
            "201" => SuccessActionStatus::Status201,
            _ => SuccessActionStatus::Status204,
        }
    }
}

/// PostObject 操作输出
#[derive(Debug, Default)]
pub struct PostObjectOutput {
    /// ETag
    pub etag: Option<String>,
    /// Object 的 URL
    pub location: Option<String>,
    /// Bucket 名称
    pub bucket: Option<String>,
    /// Object 名称
    pub key: Option<String>,
    /// 版本 ID
    pub version_id: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// CRC64 校验值
    pub hash_crc64ecma: Option<String>,
    /// Content-MD5
    pub content_md5: Option<String>,
    /// 服务端加密算法
    pub server_side_encryption: Option<String>,
}

impl PostObjectInput {
    /// 创建新的 PostObjectInput 构建器
    pub fn builder() -> PostObjectInputBuilder {
        PostObjectInputBuilder::default()
    }
}

/// PostObjectInput 构建器
#[derive(Debug, Default)]
pub struct PostObjectInputBuilder {
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
    user_metadata: HashMap<String, String>,
}

impl PostObjectInputBuilder {
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

    /// 设置文件内容
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 设置 Policy（Base64 编码）
    pub fn policy(mut self, policy: impl Into<String>) -> Self {
        self.policy = Some(policy.into());
        self
    }

    /// 设置 V1 签名
    pub fn signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }

    /// 设置 OSSAccessKeyId
    pub fn oss_access_key_id(mut self, oss_access_key_id: impl Into<String>) -> Self {
        self.oss_access_key_id = Some(oss_access_key_id.into());
        self
    }

    /// 设置 V4 签名
    pub fn x_oss_signature(mut self, x_oss_signature: impl Into<String>) -> Self {
        self.x_oss_signature = Some(x_oss_signature.into());
        self
    }

    /// 设置 V4 日期
    pub fn x_oss_date(mut self, x_oss_date: impl Into<String>) -> Self {
        self.x_oss_date = Some(x_oss_date.into());
        self
    }

    /// 设置 V4 Credential
    pub fn x_oss_credential(mut self, x_oss_credential: impl Into<String>) -> Self {
        self.x_oss_credential = Some(x_oss_credential.into());
        self
    }

    /// 设置安全令牌
    pub fn x_oss_security_token(mut self, x_oss_security_token: impl Into<String>) -> Self {
        self.x_oss_security_token = Some(x_oss_security_token.into());
        self
    }

    /// 设置成功后跳转 URL
    pub fn success_action_redirect(mut self, success_action_redirect: impl Into<String>) -> Self {
        self.success_action_redirect = Some(success_action_redirect.into());
        self
    }

    /// 设置成功后返回状态码
    pub fn success_action_status(mut self, success_action_status: SuccessActionStatus) -> Self {
        self.success_action_status = Some(success_action_status);
        self
    }

    /// 设置 Content-Disposition
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.content_disposition = Some(content_disposition.into());
        self
    }

    /// 设置 Content-Type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// 设置 Cache-Control
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.cache_control = Some(cache_control.into());
        self
    }

    /// 设置 Content-Encoding
    pub fn content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.content_encoding = Some(content_encoding.into());
        self
    }

    /// 设置 Expires
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }

    /// 设置 Object 访问权限
    pub fn x_oss_object_acl(mut self, x_oss_object_acl: impl Into<String>) -> Self {
        self.x_oss_object_acl = Some(x_oss_object_acl.into());
        self
    }

    /// 设置存储类型
    pub fn x_oss_storage_class(mut self, x_oss_storage_class: impl Into<String>) -> Self {
        self.x_oss_storage_class = Some(x_oss_storage_class.into());
        self
    }

    /// 设置服务端加密算法
    pub fn x_oss_server_side_encryption(mut self, x_oss_server_side_encryption: impl Into<String>) -> Self {
        self.x_oss_server_side_encryption = Some(x_oss_server_side_encryption.into());
        self
    }

    /// 设置 KMS 密钥 ID
    pub fn x_oss_server_side_encryption_key_id(mut self, x_oss_server_side_encryption_key_id: impl Into<String>) -> Self {
        self.x_oss_server_side_encryption_key_id = Some(x_oss_server_side_encryption_key_id.into());
        self
    }

    /// 设置服务端数据加密算法
    pub fn x_oss_server_side_data_encryption(mut self, x_oss_server_side_data_encryption: impl Into<String>) -> Self {
        self.x_oss_server_side_data_encryption = Some(x_oss_server_side_data_encryption.into());
        self
    }

    /// 设置是否禁止覆盖同名 Object
    pub fn x_oss_forbid_overwrite(mut self, x_oss_forbid_overwrite: bool) -> Self {
        self.x_oss_forbid_overwrite = Some(x_oss_forbid_overwrite);
        self
    }

    /// 添加用户自定义元数据
    pub fn user_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.user_metadata.insert(key.into(), value.into());
        self
    }

    /// 构建 PostObjectInput
    pub fn build(self) -> Result<PostObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let body = self.body.ok_or("body is required")?;

        Ok(PostObjectInput {
            bucket,
            key,
            body,
            policy: self.policy,
            signature: self.signature,
            oss_access_key_id: self.oss_access_key_id,
            x_oss_signature: self.x_oss_signature,
            x_oss_date: self.x_oss_date,
            x_oss_credential: self.x_oss_credential,
            x_oss_security_token: self.x_oss_security_token,
            success_action_redirect: self.success_action_redirect,
            success_action_status: self.success_action_status,
            content_disposition: self.content_disposition,
            content_type: self.content_type,
            cache_control: self.cache_control,
            content_encoding: self.content_encoding,
            expires: self.expires,
            x_oss_object_acl: self.x_oss_object_acl,
            x_oss_storage_class: self.x_oss_storage_class,
            x_oss_server_side_encryption: self.x_oss_server_side_encryption,
            x_oss_server_side_encryption_key_id: self.x_oss_server_side_encryption_key_id,
            x_oss_server_side_data_encryption: self.x_oss_server_side_data_encryption,
            x_oss_forbid_overwrite: self.x_oss_forbid_overwrite,
            user_metadata: self.user_metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = PostObjectInput::builder()
            .bucket("my-bucket")
            .key("my-object.txt")
            .body(b"test content".to_vec())
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object.txt");
        assert_eq!(input.body, b"test content");
    }

    #[test]
    fn test_input_builder_with_options() {
        let input = PostObjectInput::builder()
            .bucket("my-bucket")
            .key("my-object.txt")
            .body(b"test content".to_vec())
            .content_type("text/plain")
            .success_action_status(SuccessActionStatus::Status200)
            .user_metadata("uuid", "my-uuid")
            .build()
            .unwrap();

        assert_eq!(input.content_type, Some("text/plain".to_string()));
        assert_eq!(input.success_action_status, Some(SuccessActionStatus::Status200));
        assert_eq!(input.user_metadata.get("uuid"), Some(&"my-uuid".to_string()));
    }

    #[test]
    fn test_success_action_status() {
        assert_eq!(SuccessActionStatus::Status200.to_string(), "200");
        assert_eq!(SuccessActionStatus::Status201.to_string(), "201");
        assert_eq!(SuccessActionStatus::Status204.to_string(), "204");
        
        assert_eq!(SuccessActionStatus::from("200"), SuccessActionStatus::Status200);
        assert_eq!(SuccessActionStatus::from("201"), SuccessActionStatus::Status201);
        assert_eq!(SuccessActionStatus::from("204"), SuccessActionStatus::Status204);
        assert_eq!(SuccessActionStatus::from("invalid"), SuccessActionStatus::Status204);
    }

    #[test]
    fn test_input_builder_missing_bucket() {
        let result = PostObjectInput::builder()
            .key("my-object.txt")
            .body(b"test".to_vec())
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "bucket is required");
    }

    #[test]
    fn test_output_default() {
        let output = PostObjectOutput::default();

        assert_eq!(output.etag, None);
        assert_eq!(output.location, None);
        assert_eq!(output.bucket, None);
        assert_eq!(output.key, None);
        assert_eq!(output.version_id, None);
    }
}