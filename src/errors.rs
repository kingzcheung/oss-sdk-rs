//! Copyright The NoXF/oss-rust-sdk Authors
//! Copyright The iFREEGROUP/oss-sdk-rs Contributors

use super::model::error::Error as ErrorResponse;
use bytes::{Buf, Bytes};
use hmac::digest::InvalidLength;
use quick_xml::Error as QxmlError;
use reqwest::Error as ReqwestError;
use reqwest::{header::InvalidHeaderName as HttpInvalidHeaderNameError, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Error as IoError, Read};
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OSSError {
    #[error("object operation is not valid, status:{status_code:?}, message:{message:?}")]
    Object {
        status_code: StatusCode,
        message: String,
        raw_response:Value,
    },
    #[error("io error")]
    Io(#[from] IoError),
    #[error("string error")]
    String(#[from] FromUtf8Error),
    #[error("reqwest error")]
    Reqwest(#[from] ReqwestError),
    #[error("qxml error")]
    Qxml(#[from] QxmlError),
    #[error("parse xml error")]
    XmlParse(#[from] serde_xml_rs::Error),
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
        code:String,
        message:String,
        request_id:String,
        host_id:String,
        string_to_sign:String,
        canonical_request_bytes:String,
        ec:String,
        recommend_doc:String,
    },
    #[error("invalid header name")]
    InvalidHeaderName(#[from]HttpInvalidHeaderNameError),
    #[error("invalid header value")]
    InvalidHeaderValue(#[from]reqwest::header::InvalidHeaderValue)
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

pub fn status_to_bytes<'de, T>(status: StatusCode, b: Bytes) -> Result<T, OSSError>
where
    T: Deserialize<'de> + Default + From<Bytes>,
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
            Ok(b.into())
        }
        StatusCode::BAD_REQUEST | StatusCode::FORBIDDEN | StatusCode::CONFLICT => {
            let er: ErrorResponse = serde_xml_rs::from_reader(b.reader())?;
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

pub fn status_to_response<'de, T>(status: StatusCode, text: String) -> Result<T, OSSError>
where
    T: Deserialize<'de> + Default,
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
            let mut r = T::default();
            if !text.is_empty() {
                r = serde_xml_rs::from_str(&text)?;
                Ok(r)
            } else {
                Ok(r)
            }
        }
        StatusCode::BAD_REQUEST | StatusCode::FORBIDDEN | StatusCode::CONFLICT => {
            let er: ErrorResponse = serde_xml_rs::from_str(&text)?;
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
