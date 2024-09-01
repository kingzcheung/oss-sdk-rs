use std::sync::Arc;

use _put_object_input::{
    ContentDisposition, ContentEncoding, ObjectAcl, PutObjectInput, PutObjectInputBuilder,
    StorageClass,
};
use _put_object_output::PutObjectOutput;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_DISPOSITION, CONTENT_ENCODING};

use crate::{
    client::Handle,
    errors::{status_to_response, OSSError},
};

pub mod _put_object_input;
pub mod _put_object_output;

#[derive(Debug)]
pub struct PutObjectBuilder {
    inner: PutObjectInputBuilder,
    handle: Arc<Handle>,
}

impl PutObjectBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }
    pub fn key(mut self, key: &str) -> Self {
        self.inner = self.inner.key(key);
        self
    }

    pub fn bucket(mut self, bucket: &str) -> Self {
        self.inner = self.inner.bucket(bucket);
        self
    }
    pub fn body(mut self, body: &[u8]) -> Self {
        self.inner = self.inner.body(body);
        self
    }

    pub fn content_disposition(mut self, input: ContentDisposition) -> Self {
        self.inner = self.inner.content_disposition(input);
        self
    }

    pub fn content_encoding(mut self, input: ContentEncoding) -> Self {
        self.inner = self.inner.content_encoding(input);
        self
    }
    pub fn forbid_overwrite(mut self, forbid_overwrite: bool) -> Self {
        self.inner = self.inner.forbid_overwrite(forbid_overwrite);
        self
    }

    pub fn object_acl(mut self, object_acl: ObjectAcl) -> Self {
        self.inner = self.inner.object_acl(object_acl);
        self
    }

    pub fn storage_class(mut self, storage_class: StorageClass) -> Self {
        self.inner = self.inner.storage_class(storage_class);
        self
    }

    pub async fn send(self) -> Result<PutObjectOutput, OSSError> {
        let input = self.inner.build();
        crate::operation::put_object::PutObject::run_operation(input, self.handle).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct PutObject;

impl PutObject {
    fn update_headers(input: &PutObjectInput, headers: &mut HeaderMap) {
        let object_acl = input
            .object_acl
            .clone()
            .unwrap_or_else(|| ObjectAcl::default().to_string());
        headers.insert(
            "x-oss-object-acl",
            HeaderValue::from_str(&object_acl).unwrap(),
        );

        let storage_class = input
            .storage_class
            .clone()
            .unwrap_or_else(|| StorageClass::default().to_string());
        headers.insert(
            "x-oss-storage-class",
            HeaderValue::from_str(&storage_class).unwrap(),
        );

        if let Some(content_disposition) = &input.content_disposition {
            headers.insert(
                CONTENT_DISPOSITION,
                HeaderValue::from_str(content_disposition.to_string().as_str()).unwrap(),
            );
        }
        if let Some(content_encoding) = &input.content_encoding {
            let val = HeaderValue::from_str(content_encoding.to_string().as_str()).unwrap();
            headers.insert(CONTENT_ENCODING, val);
        }
        if input.forbid_overwrite {
            headers.insert(
                "x-oss-forbid-overwrite",
                HeaderValue::from_str(input.forbid_overwrite.to_string().as_str()).unwrap(),
            );
        }
    }

    async fn run_operation(
        input: PutObjectInput,
        handle: Arc<Handle>,
    ) -> Result<PutObjectOutput, OSSError> {
        let object_key = &input.key;
        let mut headers = HeaderMap::new();
        Self::update_headers(&input, &mut headers);

        let uri = format!("/{}", object_key);
        let req = handle.build_request(
            crate::client::RequestType::Put,
            &uri,
            Some(&input.bucket),
            Some(object_key),
            headers,
            None,
        );
        let resp = req.body(input.body).send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        dbg!(&text);

        status_to_response(status, text)
    }
}
