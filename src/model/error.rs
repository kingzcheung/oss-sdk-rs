use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Default)]
pub struct Error {
    #[serde(rename(serialize = "code", deserialize = "Code"))]
    pub code: String,

    #[serde(rename(serialize = "message", deserialize = "Message"))]
    pub message: String,

    #[serde(rename(serialize = "request_id", deserialize = "RequestId"))]
    pub request_id: String,

    #[serde(rename(serialize = "host_id", deserialize = "HostId"))]
    pub host_id: String,

    #[serde(rename(serialize = "argument_name", deserialize = "ArgumentName"))]
    pub argument_name: Option<String>,

    #[serde(rename(serialize = "ec", deserialize = "EC"))]
    pub ec: String,
}