//! OSS 客户端模块
//! 提供 AWS SDK 风格的客户端实现

mod abort_multipart_upload;
mod append_object;
mod complete_multipart_upload;
mod copy_object;
mod delete_multiple_objects;
mod delete_object;
mod describe_regions;
mod get_bucket_info;
mod get_bucket_location;
mod get_bucket_stat;
mod get_object;
mod get_object_acl;
mod get_object_meta;
mod get_symlink;
mod head_object;
mod initiate_multipart_upload;
mod list_buckets;
mod list_multipart_uploads;
mod list_objects;
mod list_parts;
mod post_object;
mod put_object;
mod put_object_acl;
mod put_symlink;
mod restore_object;
mod seal_append_object;
mod upload_part;
mod upload_part_copy;

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

pub use abort_multipart_upload::AbortMultipartUploadFluentBuilder;
pub use append_object::AppendObjectFluentBuilder;
pub use complete_multipart_upload::CompleteMultipartUploadFluentBuilder;
pub use copy_object::CopyObjectFluentBuilder;
pub use delete_multiple_objects::DeleteMultipleObjectsFluentBuilder;
pub use delete_object::DeleteObjectFluentBuilder;
pub use describe_regions::DescribeRegionsFluentBuilder;
pub use get_bucket_info::GetBucketInfoFluentBuilder;
pub use get_bucket_location::GetBucketLocationFluentBuilder;
pub use get_bucket_stat::GetBucketStatFluentBuilder;
pub use get_object::GetObjectFluentBuilder;
pub use get_object_acl::GetObjectAclFluentBuilder;
pub use get_object_meta::GetObjectMetaFluentBuilder;
pub use get_symlink::GetSymlinkFluentBuilder;
pub use head_object::HeadObjectFluentBuilder;
pub use initiate_multipart_upload::InitiateMultipartUploadFluentBuilder;
pub use list_buckets::ListBucketsFluentBuilder;
pub use list_multipart_uploads::ListMultipartUploadsFluentBuilder;
pub use list_objects::ListObjectsFluentBuilder;
pub use list_parts::ListPartsFluentBuilder;
pub use post_object::PostObjectFluentBuilder;
pub use put_object::PutObjectFluentBuilder;
pub use put_object_acl::PutObjectAclFluentBuilder;
pub use put_symlink::PutSymlinkFluentBuilder;
pub use restore_object::RestoreObjectFluentBuilder;
pub use seal_append_object::SealAppendObjectFluentBuilder;
pub use upload_part::UploadPartFluentBuilder;
pub use upload_part_copy::UploadPartCopyFluentBuilder;

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

    /// DeleteMultipleObjects 操作
    /// 删除同一个存储空间（Bucket）中的多个文件（Object）
    /// 单次请求最多允许删除 1000 个文件
    pub fn delete_multiple_objects(&self) -> DeleteMultipleObjectsFluentBuilder {
        DeleteMultipleObjectsFluentBuilder::new(self.handle.clone())
    }

    /// HeadObject 操作
    pub fn head_object(&self) -> HeadObjectFluentBuilder {
        HeadObjectFluentBuilder::new(self.handle.clone())
    }

    /// GetObjectMeta 操作
    /// 获取文件的元数据信息，包括 ETag、Size、LastModified 信息，不返回文件内容
    pub fn get_object_meta(&self) -> GetObjectMetaFluentBuilder {
        GetObjectMetaFluentBuilder::new(self.handle.clone())
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

    /// AppendObject 操作
    /// 以追加写的方式上传文件
    pub fn append_object(&self) -> AppendObjectFluentBuilder {
        AppendObjectFluentBuilder::new(self.handle.clone())
    }

    /// SealAppendObject 操作
    /// 停止对某个 Appendable Object 继续追加内容，并将其转为非追加状态
    pub fn seal_append_object(&self) -> SealAppendObjectFluentBuilder {
        SealAppendObjectFluentBuilder::new(self.handle.clone())
    }

    /// PostObject 操作
    /// 通过 HTML 表单上传的方式将文件（Object）上传到指定存储空间（Bucket）
    pub fn post_object(&self) -> PostObjectFluentBuilder {
        PostObjectFluentBuilder::new(self.handle.clone())
    }

    /// RestoreObject 操作
    /// 解冻归档、冷归档、深度冷归档类型的 Object
    pub fn restore_object(&self) -> RestoreObjectFluentBuilder {
        RestoreObjectFluentBuilder::new(self.handle.clone())
    }

    /// InitiateMultipartUpload 操作
    /// 初始化分片上传任务，获取 UploadId
    pub fn initiate_multipart_upload(&self) -> InitiateMultipartUploadFluentBuilder {
        InitiateMultipartUploadFluentBuilder::new(self.handle.clone())
    }

    /// UploadPart 操作
    /// 根据指定的 Object 名和 uploadId 来分片上传数据
    pub fn upload_part(&self) -> UploadPartFluentBuilder {
        UploadPartFluentBuilder::new(self.handle.clone())
    }

    /// UploadPartCopy 操作
    /// 从一个已存在的 Object 中拷贝数据来上传一个 Part
    pub fn upload_part_copy(&self) -> UploadPartCopyFluentBuilder {
        UploadPartCopyFluentBuilder::new(self.handle.clone())
    }

    /// CompleteMultipartUpload 操作
    /// 完成分片上传，将所有已上传的 Part 合并成一个完整的 Object
    pub fn complete_multipart_upload(&self) -> CompleteMultipartUploadFluentBuilder {
        CompleteMultipartUploadFluentBuilder::new(self.handle.clone())
    }

    /// AbortMultipartUpload 操作
    /// 取消 MultipartUpload 事件并删除对应的 Part 数据
    pub fn abort_multipart_upload(&self) -> AbortMultipartUploadFluentBuilder {
        AbortMultipartUploadFluentBuilder::new(self.handle.clone())
    }

    /// ListMultipartUploads 操作
    /// 列举所有执行中的 Multipart Upload 事件
    pub fn list_multipart_uploads(&self) -> ListMultipartUploadsFluentBuilder {
        ListMultipartUploadsFluentBuilder::new(self.handle.clone())
    }

    /// ListParts 操作
    /// 列举指定 Upload ID 所属的所有已经上传成功 Part
    pub fn list_parts(&self) -> ListPartsFluentBuilder {
        ListPartsFluentBuilder::new(self.handle.clone())
    }

    /// PutObjectACL 操作
    /// 修改文件（Object）的访问权限（ACL）
    pub fn put_object_acl(&self) -> PutObjectAclFluentBuilder {
        PutObjectAclFluentBuilder::new(self.handle.clone())
    }

    /// GetObjectACL 操作
    /// 获取存储空间（Bucket）下某个文件（Object）的访问权限（ACL）
    pub fn get_object_acl(&self) -> GetObjectAclFluentBuilder {
        GetObjectAclFluentBuilder::new(self.handle.clone())
    }

    /// PutSymlink 操作
    /// 为 OSS 的目标文件（TargetObject）创建软链接（Symlink）
    pub fn put_symlink(&self) -> PutSymlinkFluentBuilder {
        PutSymlinkFluentBuilder::new(self.handle.clone())
    }

    /// GetSymlink 操作
    /// 获取软链接信息
    pub fn get_symlink(&self) -> GetSymlinkFluentBuilder {
        GetSymlinkFluentBuilder::new(self.handle.clone())
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
