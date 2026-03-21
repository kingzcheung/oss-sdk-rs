//! Region 类型定义
//! 表示 OSS 区域

use std::fmt;
use std::str::FromStr;

/// OSS 区域
/// 注意：OSS V4 签名要求的区域格式是 cn-hangzhou，而不是 oss-cn-hangzhou
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Region(String);

impl Region {
    /// 创建新的 Region
    pub fn new(region: impl Into<String>) -> Self {
        Self(region.into())
    }

    /// 获取 region 字符串
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费并返回内部字符串
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for Region {
    fn default() -> Self {
        // OSS V4 签名要求的区域格式是 cn-hangzhou，而不是 oss-cn-hangzhou
        Self("cn-hangzhou".to_string())
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Region {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<String> for Region {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for Region {
    fn as_ref(&self) -> &str {
        &self.0
    }
}