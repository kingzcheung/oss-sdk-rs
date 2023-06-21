use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
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
    pub argument_name: String,

    #[serde(rename = "EC")]
    pub ec: String,
}