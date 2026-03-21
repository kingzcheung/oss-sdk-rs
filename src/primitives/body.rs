//! ByteStream 类型定义
//! 表示数据流，AWS SDK 风格

use bytes::Bytes;
use futures_util::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 数据流类型
/// 支持从 Vec<u8>, Bytes, 或 Stream 创建
pub struct ByteStream {
    inner: ByteStreamInner,
}

#[allow(unused)]
enum ByteStreamInner {
    /// 静态数据
    Static(Bytes),
    /// 动态数据
    Dynamic(Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>),
}

impl std::fmt::Debug for ByteStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            ByteStreamInner::Static(bytes) => f
                .debug_struct("ByteStream")
                .field("type", &"static")
                .field("len", &bytes.len())
                .finish(),
            ByteStreamInner::Dynamic(_) => f
                .debug_struct("ByteStream")
                .field("type", &"dynamic")
                .finish(),
        }
    }
}

impl ByteStream {
    /// 从 Vec<u8> 创建
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            inner: ByteStreamInner::Static(Bytes::from(vec)),
        }
    }

    /// 从 Bytes 创建
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self {
            inner: ByteStreamInner::Static(bytes),
        }
    }

    /// 从静态字符串创建
    pub fn from_static(s: &'static [u8]) -> Self {
        Self {
            inner: ByteStreamInner::Static(Bytes::from_static(s)),
        }
    }

    /// 创建空的 ByteStream
    pub fn empty() -> Self {
        Self {
            inner: ByteStreamInner::Static(Bytes::new()),
        }
    }

    /// 收集所有数据到 Vec<u8>
    pub async fn collect(self) -> Result<Vec<u8>, std::io::Error> {
        match self.inner {
            ByteStreamInner::Static(bytes) => Ok(bytes.to_vec()),
            ByteStreamInner::Dynamic(mut stream) => {
                let mut result = Vec::new();
                while let Some(chunk) = futures_util::StreamExt::next(&mut stream).await {
                    result.extend_from_slice(&chunk?);
                }
                Ok(result)
            }
        }
    }

    /// 获取数据长度（如果已知）
    pub fn content_length(&self) -> Option<u64> {
        match &self.inner {
            ByteStreamInner::Static(bytes) => Some(bytes.len() as u64),
            ByteStreamInner::Dynamic(_) => None,
        }
    }

    /// 转换为 Bytes
    pub fn into_bytes(self) -> Result<Bytes, std::io::Error> {
        match self.inner {
            ByteStreamInner::Static(bytes) => Ok(bytes),
            ByteStreamInner::Dynamic(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Cannot convert dynamic stream to bytes synchronously",
            )),
        }
    }
}

impl From<Vec<u8>> for ByteStream {
    fn from(value: Vec<u8>) -> Self {
        Self::from_vec(value)
    }
}

impl From<Bytes> for ByteStream {
    fn from(value: Bytes) -> Self {
        Self::from_bytes(value)
    }
}

impl From<&'static [u8]> for ByteStream {
    fn from(value: &'static [u8]) -> Self {
        Self::from_static(value)
    }
}

impl From<&'static str> for ByteStream {
    fn from(value: &'static str) -> Self {
        Self::from_static(value.as_bytes())
    }
}

impl Default for ByteStream {
    fn default() -> Self {
        Self::empty()
    }
}

impl Stream for ByteStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.get_mut().inner {
            ByteStreamInner::Static(bytes) => {
                if bytes.is_empty() {
                    Poll::Ready(None)
                } else {
                    let bytes = std::mem::take(bytes);
                    Poll::Ready(Some(Ok(bytes)))
                }
            }
            ByteStreamInner::Dynamic(stream) => stream.as_mut().poll_next(cx),
        }
    }
}
