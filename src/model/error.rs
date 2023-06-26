use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Default)]
pub struct Error {
    #[serde(rename = "Code")]
    pub code: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "RequestId")]
    pub request_id: String,

    #[serde(rename = "HostId")]
    pub host_id: String,

    #[serde(rename = "ArgumentName")]
    pub argument_name: Option<String>,

    #[serde(rename = "EC")]
    pub ec: String,
}