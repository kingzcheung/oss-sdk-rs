//! Copyright The NoXF/oss-rust-sdk Authors
//! Copyright The iFREEGROUP/oss-rust-sdk Contributors

use std::collections::HashMap;

use crate::{
    oss::{ObjectMeta, RequestType},
    prelude::{OSS}, errors::status_to_response, model::object::ListBucketResult,
};

use super::errors::{OSSError};

use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait ObjectAPI {
    async fn list_object<S, H, R>(&self, headers: H, resources: R) -> Result<ListBucketResult, OSSError>
    where
        S: AsRef<str>,
        H: Into<Option<HashMap<S, S>>> + Send,
        R: Into<Option<HashMap<S, Option<S>>>> + Send;

    async fn get_object<S1, S2, H, R>(
        &self,
        object_name: S1,
        headers: H,
        resources: R,
    ) -> Result<Bytes, OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        H: Into<Option<HashMap<S2, S2>>> + Send,
        R: Into<Option<HashMap<S2, Option<S2>>>> + Send;

    async fn put_object<S1, S2, H, R>(
        &self,
        buf: &[u8],
        object_name: S1,
        headers: H,
        resources: R,
    ) -> Result<(), OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        H: Into<Option<HashMap<S2, S2>>> + Send,
        R: Into<Option<HashMap<S2, Option<S2>>>> + Send;

    async fn append_object<S1, S2, H, R>(
        &self,
        buf: &[u8],
        object_name: S1,
        headers: H,
        resources: R,
    ) -> Result<Option<u64>, OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        H: Into<Option<HashMap<S2, S2>>> + Send,
        R: Into<Option<HashMap<S2, Option<S2>>>> + Send;

    async fn copy_object_from_object<S1, S2, S3, H, R>(
        &self,
        src: S1,
        dest: S2,
        headers: H,
        resources: R,
    ) -> Result<(), OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        S3: AsRef<str> + Send,
        H: Into<Option<HashMap<S3, S3>>> + Send,
        R: Into<Option<HashMap<S3, Option<S3>>>> + Send;

    async fn delete_object<S>(&self, object_name: S) -> Result<(), OSSError>
    where
        S: AsRef<str> + Send;

    async fn head_object<S>(&self, object_name: S) -> Result<ObjectMeta, OSSError>
    where
        S: AsRef<str> + Send;
}

#[async_trait]
impl<'a> ObjectAPI for OSS<'a> {
    async fn list_object<S, H, R>(&self, headers: H, resources: R) -> Result<ListBucketResult, OSSError>
    where
        S: AsRef<str>,
        H: Into<Option<HashMap<S, S>>> + Send,
        R: Into<Option<HashMap<S, Option<S>>>> + Send,
    {
        let (host, headers) =
            self.build_request(RequestType::Get, String::new(), headers, resources)?;

        let resp = self.http_client.get(host).headers(headers).send().await?;

        let status = resp.status();
        let text = resp.text().await?;

        status_to_response(status, text)

    }
    async fn get_object<S1, S2, H, R>(
        &self,
        object_name: S1,
        headers: H,
        resources: R,
    ) -> Result<Bytes, OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        H: Into<Option<HashMap<S2, S2>>> + Send,
        R: Into<Option<HashMap<S2, Option<S2>>>> + Send,
    {
        let (host, headers) =
            self.build_request(RequestType::Get, object_name, headers, resources)?;

        let resp = self.http_client.get(&host).headers(headers).send().await?;

        let status = resp.status();

        if status.is_success() {
            Ok(resp.bytes().await?)
        } else {
            Err(OSSError::Object { status_code: status, message: "get object fail".into() })
        }
    }

    async fn append_object<S1, S2, H, R>(
        &self,
        buf: &[u8],
        object_name: S1,
        headers: H,
        resources: R,
    ) -> Result<Option<u64>, OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        H: Into<Option<HashMap<S2, S2>>> + Send,
        R: Into<Option<HashMap<S2, Option<S2>>>> + Send,
    {
        // let mut resources:HashMap<&str, Option<&str>>= HashMap::new();
        // resources.insert("append", None);
        // resources.insert("position", Some("1"));

        let (host, headers) =
            self.build_request(RequestType::Post, object_name, headers, resources)?;
        let resp = self
            .http_client
            .post(&host)
            .headers(headers)
            .body(buf.to_owned())
            .send()
            .await?;
        let status = resp.status();

        let resp_headers = resp.headers();
        if status.is_success() {
            let next_position = if let Some(next) = resp_headers.get("x-oss-next-append-position") {
                let next = String::from_utf8_lossy(next.as_bytes()).to_string();
                match next.parse::<u64>() {
                    Ok(u) => Some(u),
                    Err(_) => None,
                }
            } else {
                None
            };
            Ok(next_position)
        } else {
            Err(OSSError::Object { status_code: status, message: "append object fail".into() })
        }
    }

    async fn put_object<S1, S2, H, R>(
        &self,
        buf: &[u8],
        object_name: S1,
        headers: H,
        resources: R,
    ) -> Result<(), OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        H: Into<Option<HashMap<S2, S2>>> + Send,
        R: Into<Option<HashMap<S2, Option<S2>>>> + Send,
    {
        let (host, headers) =
            self.build_request(RequestType::Put, object_name, headers, resources)?;

        let resp = self
            .http_client
            .put(&host)
            .headers(headers)
            .body(buf.to_owned())
            .send()
            .await?;
        let status = resp.status();
        let text = resp.text().await?;

        status_to_response::<()>(status, text)
        
    }

    async fn copy_object_from_object<S1, S2, S3, H, R>(
        &self,
        src: S1,
        dest: S2,
        headers: H,
        resources: R,
    ) -> Result<(), OSSError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        S3: AsRef<str> + Send,
        H: Into<Option<HashMap<S3, S3>>> + Send,
        R: Into<Option<HashMap<S3, Option<S3>>>> + Send,
    {
        let (host, mut headers) = self.build_request(RequestType::Put, dest, headers, resources)?;
        headers.insert("x-oss-copy-source", src.as_ref().parse()?);

        let resp = self.http_client.put(&host).headers(headers).send().await?;

        let status = resp.status();
        let text = resp.text().await?;
        status_to_response::<()>(status, text)
        
    }

    async fn delete_object<S>(&self, object_name: S) -> Result<(), OSSError>
    where
        S: AsRef<str> + Send,
    {
        let headers = HashMap::<String, String>::new();
        let (host, headers) =
            self.build_request(RequestType::Delete, object_name, Some(headers), None)?;

        let resp = self
            .http_client
            .delete(&host)
            .headers(headers)
            .send()
            .await?;

            let status = resp.status();
            let text = resp.text().await?;
            status_to_response::<()>(status, text)
    }

    async fn head_object<S>(&self, object_name: S) -> Result<ObjectMeta, OSSError>
    where
        S: AsRef<str> + Send,
    {
        let (host, headers) = self.build_request(
            RequestType::Head,
            object_name,
            None::<HashMap<String, String>>,
            None,
        )?;

        let resp = self.http_client.head(&host).headers(headers).send().await?;
        let status = resp.status();
        if resp.status().is_success() {
            Ok(ObjectMeta::from_header_map(resp.headers())?)
        } else {
            Err(OSSError::Object { status_code: status, message: "head object error".into() })
        }
    }
}
