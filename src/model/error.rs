use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
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

    #[serde(rename(serialize = "recommend_doc", deserialize = "RecommendDoc"))]
    pub recommend_doc: Option<String>,
    #[serde(rename(
        serialize = "canonical_request_bytes",
        deserialize = "CanonicalRequestBytes"
    ))]
    pub canonical_request_bytes: Option<String>,

    #[serde(rename(serialize = "ossaccess_key_id", deserialize = "OSSAccessKeyId"))]
    pub ossaccess_key_id: Option<String>,

    #[serde(rename(serialize = "signature_provided", deserialize = "SignatureProvided"))]
    pub signature_provided: Option<String>,

    #[serde(rename(serialize = "string_to_sign", deserialize = "StringToSign"))]
    pub string_to_sign: Option<String>,
    
    #[serde(rename(serialize = "string_to_sign_bytes", deserialize = "StringToSignBytes"))]
    pub string_to_sign_bytes: Option<String>,
}
