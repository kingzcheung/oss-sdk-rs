//! 凭证模块
//! 提供 AWS SDK 风格的凭证管理

use std::sync::Arc;

/// OSS 凭证
#[derive(Debug, Clone)]
pub struct Credentials {
    /// Access Key ID
    access_key_id: String,
    /// Access Key Secret
    access_key_secret: String,
    /// 安全令牌（可选，用于 STS）
    security_token: Option<String>,
}

impl Credentials {
    /// 创建新的凭证
    pub fn new(access_key_id: impl Into<String>, access_key_secret: impl Into<String>) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            security_token: None,
        }
    }

    /// 创建带安全令牌的凭证（STS）
    pub fn new_with_token(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        security_token: impl Into<String>,
    ) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            security_token: Some(security_token.into()),
        }
    }

    /// 获取 Access Key ID
    pub fn access_key_id(&self) -> &str {
        &self.access_key_id
    }

    /// 获取 Access Key Secret
    pub fn access_key_secret(&self) -> &str {
        &self.access_key_secret
    }

    /// 获取安全令牌
    pub fn security_token(&self) -> Option<&str> {
        self.security_token.as_deref()
    }
}

/// 凭证提供者 trait
#[async_trait::async_trait]
pub trait ProvideCredentials: Send + Sync + std::fmt::Debug {
    /// 获取凭证
    async fn provide_credentials(&self) -> Result<Credentials, CredentialsError>;
}

/// 凭证错误
#[derive(Debug, thiserror::Error)]
pub enum CredentialsError {
    /// 凭证未找到
    #[error("credentials not found")]
    NotFound,
    /// 凭证无效
    #[error("invalid credentials: {0}")]
    Invalid(String),
    /// 加载凭证失败
    #[error("failed to load credentials: {0}")]
    LoadFailed(String),
}

/// 静态凭证提供者
#[derive(Debug, Clone)]
pub struct StaticCredentialsProvider {
    credentials: Credentials,
}

impl StaticCredentialsProvider {
    /// 创建新的静态凭证提供者
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

#[async_trait::async_trait]
impl ProvideCredentials for StaticCredentialsProvider {
    async fn provide_credentials(&self) -> Result<Credentials, CredentialsError> {
        Ok(self.credentials.clone())
    }
}

impl From<Credentials> for StaticCredentialsProvider {
    fn from(credentials: Credentials) -> Self {
        Self::new(credentials)
    }
}

/// 凭证提供者包装类型
#[derive(Debug, Clone)]
pub struct CredentialsProvider(Arc<dyn ProvideCredentials>);

impl CredentialsProvider {
    /// 创建新的凭证提供者
    pub fn new(provider: impl ProvideCredentials + 'static) -> Self {
        Self(Arc::new(provider))
    }

    /// 获取凭证
    pub async fn provide_credentials(&self) -> Result<Credentials, CredentialsError> {
        self.0.provide_credentials().await
    }
}

impl From<StaticCredentialsProvider> for CredentialsProvider {
    fn from(provider: StaticCredentialsProvider) -> Self {
        Self::new(provider)
    }
}

impl From<Credentials> for CredentialsProvider {
    fn from(credentials: Credentials) -> Self {
        StaticCredentialsProvider::new(credentials).into()
    }
}
