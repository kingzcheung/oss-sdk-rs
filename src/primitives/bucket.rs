//! Bucket 类型定义
//! 表示 OSS 存储桶名称

use std::fmt;
use std::str::FromStr;

/// OSS 存储桶名称
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bucket(String);

impl Bucket {
    /// 创建新的 Bucket
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// 获取 bucket 名称
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费并返回内部字符串
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Bucket {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<String> for Bucket {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Bucket {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for Bucket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}