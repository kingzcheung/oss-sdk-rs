//! Copyright The NoXF/oss-rust-sdk Authors
//! Copyright The iFREEGROUP/oss-sdk-rs Contributors
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::header::{CONTENT_TYPE, DATE};

use base64::prelude::*;
use hmac::{Hmac, Mac};

type HmacSha1 = Hmac<sha1::Sha1>;

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
            .get(CONTENT_TYPE).map(|c| c.to_str().unwrap_or_default())
            .unwrap_or_default();
        let content_md5 = headers
            .get("Content-MD5").map(|md5| BASE64_STANDARD.encode(md5.to_str().unwrap_or_default()))
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

        let mut hasher = HmacSha1::new_from_slice(key_secret.as_bytes())
            .expect("Hmac can take key of any size, should not happned");
        hasher.update(sign_str.as_bytes());

        BASE64_STANDARD.encode(hasher.finalize().into_bytes())
    }

    fn oss_sign(
        &self,
        verb: &str,
        key_id: &str,
        key_secret: &str,
        bucket: &str,
        object: &str,
        oss_resources: &str,
        headers: &HeaderMap,
    ) -> String {
        let sign_str_base64 = self.sign(verb, key_secret, bucket, object, oss_resources, headers);
        let authorization = format!("OSS {}:{}", key_id, sign_str_base64);
        debug!("authorization: {}", authorization);
        authorization
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