use bytes::Bytes;
use serde::Deserialize;

#[derive(Debug,Deserialize,Default)]
pub struct GetObjectOutput {
    pub body:Vec<u8> ,
}

impl From<Bytes> for GetObjectOutput {
    fn from(value: Bytes) -> Self {
        Self { body: value.to_vec() }
    }
}