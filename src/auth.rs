//! Copyright The NoXF/oss-rust-sdk Authors
//! Copyright The iFREEGROUP/oss-rust-sdk Contributors
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::header::{CONTENT_TYPE, DATE};

use base64::{ engine::general_purpose, Engine};
use hmac::{Hmac, Mac};

type HmacSha1 = Hmac<sha1::Sha1>;

use crate::errors::OSSError;

use super::oss::OSS;

pub trait Auth {
    fn oss_sign(
        &self,
        verb: &str,
        bucket: &str,
        object: &str,
        oss_resources: &str,
        headers: &HeaderMap,
    ) -> Result<String, OSSError>;

    fn sign_content(&self, content: &str) -> Result<String, OSSError>;
}

impl<'a> Auth for OSS<'a> {
    fn oss_sign(
        &self,
        verb: &str,
        bucket: &str,
        object: &str,
        oss_resources: &str,
        headers: &HeaderMap,
    ) -> Result<String, OSSError> {
        let date = headers
            .get(DATE)
            .map(|d| d.to_str().unwrap_or_default())
            .unwrap_or_default();
        let content_type = headers
            .get(CONTENT_TYPE)
            .map(|c| c.to_str().unwrap_or_default())
            .unwrap_or_default();
        let content_md5 = headers
            .get("Content-MD5")
            .map(|md5| general_purpose::STANDARD.encode(md5.to_str().unwrap_or_default()))
            .unwrap_or_default();

        let mut oss_headers: Vec<(&HeaderName, &HeaderValue)> = headers
            .iter()
            .filter(|(k, _)| k.as_str().contains("x-oss-"))
            .collect();
        oss_headers.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
        let mut oss_headers_str = String::new();
        for (k, v) in oss_headers {
            oss_headers_str += &format!(
                "{}:{}\n",
                k.to_owned().as_str(),
                v.to_owned().to_str().unwrap_or("")
            );
        }

        let oss_resource_str = get_oss_resource_str(bucket, object, oss_resources);
        let sign_str = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            verb, content_md5, content_type, date, oss_headers_str, oss_resource_str
        );

        self.sign_content(sign_str.as_str())
    }

    fn sign_content(&self, content: &str) -> Result<String, OSSError> {
        let mut hasher =
            HmacSha1::new_from_slice(self.key_secret().as_bytes()).map_err(OSSError::Sign)?;
        hasher.update(content.as_bytes());

        let sign_str_base64 = general_purpose::STANDARD.encode(hasher.finalize().into_bytes());

        let authorization = format!("OSS {}:{}", self.key_id(), sign_str_base64);
        debug!("authorization: {}", authorization);
        Ok(authorization)
    }
}

#[inline]
fn get_oss_resource_str(bucket: &str, object: &str, oss_resources: &str) -> String {
    let oss_resources = if !oss_resources.is_empty() {
        String::from("?") + oss_resources
    } else {
        String::new()
    };
    if bucket.is_empty() {
        format!("/{}{}", bucket, oss_resources)
    } else {
        format!("/{}/{}{}", bucket, object, oss_resources)
    }
}
