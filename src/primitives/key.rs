//! Key 类型定义
//! 表示 OSS 对象键

use std::fmt;
use std::str::FromStr;

/// OSS 对象键
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(String);

impl Key {
    /// 创建新的 Key
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    /// 获取 key 字符串
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费并返回内部字符串
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Key {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
