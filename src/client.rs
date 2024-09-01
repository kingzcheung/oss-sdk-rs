use core::time;
use std::{fmt::Display, sync::Arc};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    RequestBuilder, Url,
};

use crate::{authv4::SignerV4, config::Config};

pub mod get_object;
pub mod put_object;

#[derive(Debug, Clone)]
pub enum RequestType {
    Get,
    Put,
    Delete,
    Head,
    Post,
}

impl From<RequestType> for reqwest::Method {
    fn from(value: RequestType) -> Self {
        match value {
            RequestType::Get => Self::GET,
            RequestType::Put => Self::PUT,
            RequestType::Delete => Self::DELETE,
            RequestType::Head => Self::HEAD,
            RequestType::Post => Self::POST,
        }
    }
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestType::Get => write!(f, "GET"),
            RequestType::Put => write!(f, "PUT"),
            RequestType::Delete => write!(f, "DELETE"),
            RequestType::Head => write!(f, "HEAD"),
            RequestType::Post => write!(f, "POST"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Handle {
    pub(crate) conf: Config,
    pub(crate) http_client: reqwest::Client,
}

impl Handle {

    
    pub fn build_request<S: AsRef<str>>(
        &self,
        method: RequestType,
        uri: S,
        bucket: Option<S>,
        object_key: Option<S>,
        headers: HeaderMap,
        raw_query: Option<S>,
    ) -> RequestBuilder {
        let region = self.conf.region().map(|x| x.as_ref()).unwrap_or_default();
        let raw_query = raw_query
            .map(|x| x.as_ref().to_string())
            .unwrap_or_default();
        let m: reqwest::Method = method.clone().into();

        let mut common_headers = HeaderMap::new();
        common_headers.insert("x-oss-content-sha256", HeaderValue::from_static("UNSIGNED-PAYLOAD"));

        if let Some(bucket) = &bucket {
            let header_host = get_header_host(&self.conf.endpoint, bucket.as_ref());
            common_headers.insert("Host", HeaderValue::from_str(&header_host).unwrap());
        }
        common_headers.extend(headers);

        let v4 = SignerV4::new(
            &common_headers,
            method,
            &self.conf.access_key_id,
            &self.conf.access_key_secret,
            &raw_query,
            region,
        );
        let additional_headers = &[];
        let sign_headers = v4.sign(
            bucket.map(|x| x.as_ref().to_string()).as_deref(),
            object_key.map(|x| x.as_ref().to_string()).as_deref(),
            additional_headers,
        );

        common_headers.extend(sign_headers);


        let url = format!("{}{}", &self.conf.endpoint, uri.as_ref());

        self.http_client.request(m, url).headers(common_headers)
    }
}

fn get_header_host(endpoint: &str, bucket: &str) -> String {
    let u = Url::parse(endpoint).unwrap();
    format!("{}.{}", bucket, u.host().unwrap())
}

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) handle: Arc<Handle>,
}

impl Client {
    pub fn from_conf(conf: Config) -> Self {
        let http_client = reqwest::ClientBuilder::new()
            .connect_timeout(time::Duration::from_secs(10))
            .build()
            .unwrap();
        let handle = Handle { conf, http_client };
        Self {
            handle: Arc::new(handle),
        }
    }
}
