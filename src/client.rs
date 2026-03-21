//! OSS 客户端模块
//! 提供 AWS SDK 风格的客户端实现

mod copy_object;
mod delete_object;
mod describe_regions;
mod get_bucket_info;
mod get_bucket_location;
mod get_bucket_stat;
mod get_object;
mod head_object;
mod list_buckets;
mod list_objects;
mod put_object;

use std::sync::Arc;
use std::time::Duration;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    RequestBuilder, Url,
};

use crate::authv4::SignerV4;
use crate::config::Config;
use crate::credentials::Credentials;
use crate::errors::OSSError;

pub use copy_object::CopyObjectFluentBuilder;
pub use delete_object::DeleteObjectFluentBuilder;
pub use describe_regions::DescribeRegionsFluentBuilder;
pub use get_bucket_info::GetBucketInfoFluentBuilder;
pub use get_bucket_location::GetBucketLocationFluentBuilder;
pub use get_bucket_stat::GetBucketStatFluentBuilder;
pub use get_object::GetObjectFluentBuilder;
pub use head_object::HeadObjectFluentBuilder;
pub use list_buckets::ListBucketsFluentBuilder;
pub use list_objects::ListObjectsFluentBuilder;
pub use put_object::PutObjectFluentBuilder;

/// HTTP 请求方法
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Head,
}

impl From<HttpMethod> for reqwest::Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => Self::GET,
            HttpMethod::Put => Self::PUT,
            HttpMethod::Post => Self::POST,
            HttpMethod::Delete => Self::DELETE,
            HttpMethod::Head => Self::HEAD,
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Head => write!(f, "HEAD"),
        }
    }
}

/// 内部句柄
#[derive(Debug)]
pub struct Handle {
    config: Config,
    http_client: reqwest::Client,
    credentials: Credentials,
}

impl Handle {
    /// 构建请求
    pub fn build_request(
        &self,
        method: HttpMethod,
        uri: &str,
        bucket: Option<&str>,
        object_key: Option<&str>,
        headers: HeaderMap,
        query: Option<&str>,
    ) -> Result<RequestBuilder, OSSError> {
        let region = self.config.region().as_str();
        let query = query.unwrap_or("");
        let m: reqwest::Method = method.into();

        let mut common_headers = HeaderMap::new();
        common_headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );

        if let Some(bucket) = bucket {
            let endpoint = self.config.endpoint().unwrap_or("");
            let header_host = get_header_host(endpoint, bucket);
            common_headers.insert(
                "Host",
                HeaderValue::from_str(&header_host).map_err(|e| OSSError::InvalidHeaderValue(e))?,
            );
        }
        common_headers.extend(headers);

        let v4 = SignerV4::new(
            &common_headers,
            method,
            self.credentials.access_key_id(),
            self.credentials.access_key_secret(),
            query,
            region,
        );
        let additional_headers = &[];
        let sign_headers = v4.sign(
            bucket.map(|s| s.to_string()).as_deref(),
            object_key.map(|s| s.to_string()).as_deref(),
            additional_headers,
        );

        common_headers.extend(sign_headers);

        let endpoint = self.config.endpoint().unwrap_or("");
        let mut url = format!("{}{}", endpoint, uri);

        // 添加查询参数
        if !query.is_empty() {
            url.push('?');
            url.push_str(query);
        }

        Ok(self.http_client.request(m, url).headers(common_headers))
    }
}

fn get_header_host(endpoint: &str, bucket: &str) -> String {
    if let Ok(u) = Url::parse(endpoint) {
        if let Some(host) = u.host_str() {
            return format!("{}.{}", bucket, host);
        }
    }
    format!("{}.oss-cn-hangzhou.aliyuncs.com", bucket)
}

/// OSS 客户端
#[derive(Debug, Clone)]
pub struct Client {
    handle: Arc<Handle>,
}

impl Client {
    /// 从配置创建客户端
    pub fn from_config(config: Config) -> Result<Self, OSSError> {
        let credentials = config
            .credentials()
            .cloned()
            .ok_or_else(|| OSSError::Credentials("credentials not set".to_string()))?;

        let mut builder = reqwest::ClientBuilder::new()
            .connect_timeout(config.connect_timeout().unwrap_or(Duration::from_secs(10)));

        if let Some(timeout) = config.timeout() {
            builder = builder.timeout(timeout);
        }

        let http_client = builder.build().map_err(|e| OSSError::Reqwest(e))?;

        let handle = Handle {
            config,
            http_client,
            credentials,
        };

        Ok(Self {
            handle: Arc::new(handle),
        })
    }

    /// 创建配置构建器
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// GetObject 操作
    pub fn get_object(&self) -> GetObjectFluentBuilder {
        GetObjectFluentBuilder::new(self.handle.clone())
    }

    /// PutObject 操作
    pub fn put_object(&self) -> PutObjectFluentBuilder {
        PutObjectFluentBuilder::new(self.handle.clone())
    }

    /// ListObjects 操作
    pub fn list_objects(&self) -> ListObjectsFluentBuilder {
        ListObjectsFluentBuilder::new(self.handle.clone())
    }

    /// ListBuckets（GetService）操作
    /// 获取请求者拥有的所有 Bucket 列表
    pub fn list_buckets(&self) -> ListBucketsFluentBuilder {
        ListBucketsFluentBuilder::new(self.handle.clone())
    }

    /// DeleteObject 操作
    pub fn delete_object(&self) -> DeleteObjectFluentBuilder {
        DeleteObjectFluentBuilder::new(self.handle.clone())
    }

    /// HeadObject 操作
    pub fn head_object(&self) -> HeadObjectFluentBuilder {
        HeadObjectFluentBuilder::new(self.handle.clone())
    }

    /// CopyObject 操作
    pub fn copy_object(&self) -> CopyObjectFluentBuilder {
        CopyObjectFluentBuilder::new(self.handle.clone())
    }

    /// DescribeRegions 操作
    /// 查询所有支持地域或指定地域对应的 Endpoint 信息
    pub fn describe_regions(&self) -> DescribeRegionsFluentBuilder {
        DescribeRegionsFluentBuilder::new(self.handle.clone())
    }

    /// GetBucketInfo 操作
    /// 获取 Bucket 的详细信息
    pub fn get_bucket_info(&self) -> GetBucketInfoFluentBuilder {
        GetBucketInfoFluentBuilder::new(self.handle.clone())
    }

    /// GetBucketLocation 操作
    /// 获取 Bucket 的位置信息
    pub fn get_bucket_location(&self) -> GetBucketLocationFluentBuilder {
        GetBucketLocationFluentBuilder::new(self.handle.clone())
    }

    /// GetBucketStat 操作
    /// 获取指定 Bucket 的存储容量、文件以及 Multipart 分片数量
    pub fn get_bucket_stat(&self) -> GetBucketStatFluentBuilder {
        GetBucketStatFluentBuilder::new(self.handle.clone())
    }
}

/// 客户端构建器
#[derive(Default)]
pub struct ClientBuilder {
    config: Option<Config>,
}

impl ClientBuilder {
    /// 设置配置
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// 构建客户端
    pub fn build(self) -> Result<Client, OSSError> {
        let config = self
            .config
            .ok_or_else(|| OSSError::Config("config not set".to_string()))?;
        Client::from_config(config)
    }
}
