//! Copyright The NoXF/oss-rust-sdk Authors
//! Copyright The iFREEGROUP/oss-sdk-rs Contributors

use bytes::{Buf, Bytes};
use hmac::digest::InvalidLength;
use quick_xml::Error as QxmlError;
use reqwest::Error as ReqwestError;
use reqwest::{header::InvalidHeaderName as HttpInvalidHeaderNameError, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::io::Cursor;
use std::io::Error as IoError;
use std::string::FromUtf8Error;
use thiserror::Error;

/// OSS 错误响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
}

#[derive(Debug, Error)]
pub enum OSSError {
    #[error("object operation is not valid, status:{status_code:?}, message:{message:?}")]
    Object {
        status_code: StatusCode,
        message: String,
        raw_response: Value,
    },
    #[error("io error")]
    Io(#[from] IoError),
    #[error("string error")]
    String(#[from] FromUtf8Error),
    #[error("reqwest error")]
    Reqwest(#[from] ReqwestError),
    #[error("qxml error")]
    Qxml(#[from] QxmlError),
    #[error("parse xml error: {0}")]
    XmlParse(String),
    #[error("sign invalid length")]
    Sign(#[from] InvalidLength),
    #[error("unknown error")]
    Unknown,
    #[error("bucket not set")]
    BucketNotSet,
    #[error("key not set")]
    KeyNotSet,
    #[error("signature does not match")]
    SignatureDoesNotMatch {
        code: String,
        message: String,
        request_id: String,
        host_id: String,
        string_to_sign: String,
        canonical_request_bytes: String,
        ec: String,
        recommend_doc: String,
    },
    #[error("invalid header name")]
    InvalidHeaderName(#[from] HttpInvalidHeaderNameError),
    #[error("invalid header value")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("credentials error: {0}")]
    Credentials(String),
    #[error("config error: {0}")]
    Config(String),
}

#[derive(Serialize, Deserialize)]
pub struct OutputError {
    #[serde(rename = "Code")]
    code: String,

    #[serde(rename = "Message")]
    message: String,

    #[serde(rename = "RequestId")]
    request_id: String,

    #[serde(rename = "HostId")]
    host_id: String,

    #[serde(rename = "Authorization")]
    authorization: String,

    #[serde(rename = "EC")]
    ec: String,

    #[serde(rename = "RecommendDoc")]
    recommend_doc: String,
}

pub fn status_to_bytes<T>(status: StatusCode, b: Bytes) -> Result<T, OSSError>
where
    T: DeserializeOwned + Default + From<Bytes>,
{
    match status {
        StatusCode::OK
        | StatusCode::CREATED
        | StatusCode::ACCEPTED
        | StatusCode::NON_AUTHORITATIVE_INFORMATION
        | StatusCode::NO_CONTENT
        | StatusCode::RESET_CONTENT
        | StatusCode::PARTIAL_CONTENT
        | StatusCode::MULTI_STATUS
        | StatusCode::ALREADY_REPORTED => Ok(b.into()),
        StatusCode::BAD_REQUEST | StatusCode::FORBIDDEN | StatusCode::CONFLICT => {
            let er: ErrorResponse = quick_xml::de::from_reader(b.reader())
                .map_err(|e| OSSError::XmlParse(e.to_string()))?;
            let raw = serde_json::to_value(&er).unwrap();
            Err(OSSError::Object {
                status_code: status,
                message: er.message,
                raw_response: raw,
            })
        }
        _ => Err(OSSError::Unknown),
    }
}

pub fn status_to_response<T>(status: StatusCode, text: String) -> Result<T, OSSError>
where
    T: DeserializeOwned + Default,
{
    match status {
        StatusCode::OK
        | StatusCode::CREATED
        | StatusCode::ACCEPTED
        | StatusCode::NON_AUTHORITATIVE_INFORMATION
        | StatusCode::NO_CONTENT
        | StatusCode::RESET_CONTENT
        | StatusCode::PARTIAL_CONTENT
        | StatusCode::MULTI_STATUS
        | StatusCode::ALREADY_REPORTED => {
            if !text.is_empty() {
                let r: T = quick_xml::de::from_reader(Cursor::new(text))
                    .map_err(|e| OSSError::XmlParse(e.to_string()))?;
                Ok(r)
            } else {
                Ok(T::default())
            }
        }
        StatusCode::BAD_REQUEST | StatusCode::FORBIDDEN | StatusCode::CONFLICT => {
            let er: ErrorResponse = quick_xml::de::from_reader(Cursor::new(text))
                .map_err(|e| OSSError::XmlParse(e.to_string()))?;
            let raw = serde_json::to_value(&er).unwrap();
            Err(OSSError::Object {
                status_code: status,
                message: er.message,
                raw_response: raw,
            })
        }
        _ => Err(OSSError::Unknown),
    }
}
