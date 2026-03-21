//! 配置模块
//! 提供 AWS SDK 风格的配置管理

use std::time::Duration;

use crate::credentials::Credentials;
use crate::primitives::Region;

/// OSS 客户端配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 区域
    region: Region,
    /// 端点
    endpoint: Option<String>,
    /// 凭证
    credentials: Option<Credentials>,
    /// HTTP 超时时间
    timeout: Option<Duration>,
    /// 连接超时时间
    connect_timeout: Option<Duration>,
}

impl Config {
    /// 创建新的配置构建器
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// 获取区域
    pub fn region(&self) -> &Region {
        &self.region
    }

    /// 获取端点
    pub fn endpoint(&self) -> Option<&str> {
        self.endpoint.as_deref()
    }

    /// 获取凭证
    pub fn credentials(&self) -> Option<&Credentials> {
        self.credentials.as_ref()
    }

    /// 获取超时时间
    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// 获取连接超时时间
    pub fn connect_timeout(&self) -> Option<Duration> {
        self.connect_timeout
    }
}

/// 从 endpoint URL 中提取区域
/// 例如: https://oss-cn-guangzhou.aliyuncs.com -> cn-guangzhou
/// 注意：OSS V4 签名需要的区域格式是 cn-guangzhou，而不是 oss-cn-guangzhou
fn extract_region_from_endpoint(endpoint: &str) -> Option<String> {
    // 移除协议前缀
    let host = endpoint
        .strip_prefix("https://")
        .or_else(|| endpoint.strip_prefix("http://"))
        .unwrap_or(endpoint);
    
    // OSS endpoint 格式: oss-cn-hangzhou.aliyuncs.com
    // 提取第一个点之前的部分
    if let Some(dot_pos) = host.find('.') {
        let region_part = &host[..dot_pos];
        // 去掉 "oss-" 前缀，得到真正的区域名
        // 例如: oss-cn-guangzhou -> cn-guangzhou
        if let Some(region) = region_part.strip_prefix("oss-") {
            return Some(region.to_string());
        }
    }
    None
}

/// 配置构建器
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    region: Option<Region>,
    endpoint: Option<String>,
    credentials: Option<Credentials>,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
}

impl ConfigBuilder {
    /// 设置区域
    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// 设置端点
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// 设置凭证
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// 设置超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// 设置连接超时时间
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// 构建配置
    pub fn build(self) -> Result<Config, &'static str> {
        // 如果没有显式设置区域，尝试从 endpoint 提取
        let region = if let Some(ref region) = self.region {
            region.clone()
        } else if let Some(ref endpoint) = self.endpoint {
            // 从 endpoint 提取区域
            extract_region_from_endpoint(endpoint)
                .map(Region::new)
                .unwrap_or_default()
        } else {
            Region::default()
        };

        Ok(Config {
            region,
            endpoint: self.endpoint,
            credentials: self.credentials,
            timeout: self.timeout,
            connect_timeout: self.connect_timeout,
        })
    }
}

/// 从环境变量加载配置
pub fn from_env() -> Result<Config, &'static str> {
    let access_key_id = std::env::var("OSS_ACCESS_KEY_ID")
        .or_else(|_| std::env::var("OSS_AK"))
        .map_err(|_| "OSS_ACCESS_KEY_ID or OSS_AK not set")?;
    
    let access_key_secret = std::env::var("OSS_ACCESS_KEY_SECRET")
        .or_else(|_| std::env::var("OSS_SK"))
        .map_err(|_| "OSS_ACCESS_KEY_SECRET or OSS_SK not set")?;
    
    let region = std::env::var("OSS_REGION")
        .map(Region::new);
    
    let endpoint = std::env::var("OSS_ENDPOINT").ok();

    let credentials = Credentials::new(access_key_id, access_key_secret);

    let mut builder = Config::builder()
        .credentials(credentials);
    
    if let Ok(region) = region {
        builder = builder.region(region);
    }
    
    if let Some(endpoint) = endpoint {
        builder = builder.endpoint(endpoint);
    }
    
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_region_from_endpoint() {
        // OSS V4 签名需要的区域格式是 cn-guangzhou，而不是 oss-cn-guangzhou
        assert_eq!(
            extract_region_from_endpoint("https://oss-cn-guangzhou.aliyuncs.com"),
            Some("cn-guangzhou".to_string())
        );
        assert_eq!(
            extract_region_from_endpoint("https://oss-cn-hangzhou.aliyuncs.com"),
            Some("cn-hangzhou".to_string())
        );
        assert_eq!(
            extract_region_from_endpoint("https://oss-us-west-1.aliyuncs.com"),
            Some("us-west-1".to_string())
        );
        assert_eq!(
            extract_region_from_endpoint("http://oss-cn-shanghai.aliyuncs.com"),
            Some("cn-shanghai".to_string())
        );
        assert_eq!(
            extract_region_from_endpoint("oss-cn-shanghai.aliyuncs.com"),
            Some("cn-shanghai".to_string())
        );
    }
}