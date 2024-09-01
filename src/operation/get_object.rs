use std::sync::Arc;

use _get_object_input::GetObjectInputBuilder;
use _get_object_output::GetObjectOutput;
use reqwest::header::HeaderMap;

use crate::{client::Handle, errors::{status_to_bytes,  OSSError}};

pub mod _get_object_input;
pub mod _get_object_output;
#[derive(Debug, Clone, Default)]
pub struct GetObject;

impl GetObject {
    pub fn new() -> Self {
        Self
    }

    async fn run_operation(
        input: _get_object_input::GetObjectInput,
        handle: Arc<Handle>,
    ) -> Result<GetObjectOutput, OSSError> {
        if input.bucket.is_none() {
            return Err(OSSError::KeyNotSet);
        }

        let object_key = &input.key.unwrap_or_default();
        let headers = HeaderMap::new();

        let uri = format!("/{}", object_key);
        let req = handle.build_request(
            crate::client::RequestType::Get,
            &uri,
            input.bucket.as_ref(),
            Some(object_key),
            headers,
            None,
        );

        let resp = req.send().await?;
        let status = resp.status();
       
        let byte = resp.bytes().await?;
        // dbg!(String::from_utf8(byte.to_vec()).unwrap());
        status_to_bytes(status, byte)
    }
}
#[derive(Debug)]
pub struct GetObjectBuilder {
    inner: GetObjectInputBuilder,
    handle: Arc<Handle>,
}

impl GetObjectBuilder {
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

    pub async fn send(self) -> Result<GetObjectOutput, OSSError> {
        let input = self.inner.build();
        crate::operation::get_object::GetObject::run_operation(input, self.handle).await
        
    }
}
