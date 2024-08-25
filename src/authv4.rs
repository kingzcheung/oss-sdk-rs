use std::collections::{BTreeMap, HashMap, HashSet};

use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::{DateTime, Utc};
use hex;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, DATE};
use sha2::{Digest, Sha256};

use crate::{client::RequestType, common::{ALIYUN_V4_PREFIX, ALIYUN_V4_REQUEST, OSS_CONTENT_SHA256}};
type HmacSha256 = Hmac<Sha256>;

const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";

/// 检查给定的字符串是否是默认签名头部的一部分。
///
/// # 参数
/// * `low`: 需要检查的小写头部名称。
///
/// # 返回
/// * 如果是默认签名头部，则返回 `true`；否则返回 `false`。
fn is_default_signed_header(low: &str) -> bool {
    const OSS_HEADER_PREFIX: &str = "x-oss-"; // 假设这是预定义的常量

    low.starts_with(OSS_HEADER_PREFIX) || low == "content-type" || low == "content-md5"
}

/// 获取额外头部中符合特定条件的头部名称。
///
/// # 参数
/// * `header`: HTTP 头部。
/// * `additional_headers`: 额外头部名称的切片。
///
/// # 返回
/// * 符合条件的头部名称的切片。
fn get_common_additional_headers(header: &HeaderMap, additional_headers: &[String]) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();

    for k in additional_headers {
        let low_k = k.to_lowercase();
        if !is_default_signed_header(&low_k) && header.get(&low_k).is_some() {
            keys.push(low_k);
        }
    }

    keys.sort(); // Rust 的 sort 方法会自动按字典序排序
    keys
}
fn no_escape(c: u8) -> bool {
    matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~')
}
/// 对路径字符串进行转义处理
/// 
/// # Arguments
/// * `path` - 需要处理的路径字符串
/// * `encode_sep` - 是否对路径分隔符进行编码
///
/// # Returns
/// * 转义后的路径字符串
fn escape_path(path: &str, encode_sep: bool) -> String {
    let mut buf = String::new();
    fn no_escape(c: u8) -> bool {
    matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~')
}
    for c in path.bytes() {
        if no_escape(c) || (c == b'/' && !encode_sep) {
            buf.push(c as char);
        } else {
            buf.push_str(format!("%{:02X}",c).as_str());
        }
    }
    buf
}

/// 从请求的 URL 中提取查询参数，并将其转换为规范化的查询字符串。
///
/// # 参数
/// * `raw_query` - 原始查询字符串。
///
/// # 返回
/// * 规范化的查询字符串。
fn canonical_query(raw_query: &str) -> String {
    let query = raw_query.replace('+', "%20");
    let mut values = HashMap::new();

    let mut params: Vec<String> = query
        .split('&')
        .map(|x| {
            if let Some(idx) = x.find('=') {
                let (k, v) = x.split_at(idx);
                values.insert(k.to_string(), v.to_string());
                k.to_string()
            } else {
                "".into()
            }
        })
        .filter(|f| !f.is_empty())
        .collect();

    params.sort_unstable();

    let mut buf = String::new();
    for (i, k) in params.iter().enumerate() {
        if i > 0 {
            buf.push('&');
        }
        buf.push_str(k);
        if let Some(v) = values.get(k) {
            buf.push('=');
            buf.push_str(v);
        }
    }

    buf
}

pub struct SignerV4<'a> {
    headers: &'a HeaderMap,
    method: String,
    access_key: String,
    access_key_secret: String,
    raw_query: String,
    region:String,
}

impl<'a> SignerV4<'a> {
    pub fn new(
        headers: &'a HeaderMap,
        method:RequestType,
        access_key: &str,
        access_key_secret: &str,
        raw_query: &str,
        region: &str,
    ) -> Self {
        Self {
            headers,
            method:method.to_string(),
            access_key:access_key.to_string(),
            access_key_secret:access_key_secret.to_string(),
            raw_query:raw_query.to_string(),
            region:region.to_string(),
        }
    }

    pub fn date(&self,now:DateTime<Utc>) -> String {
        let fmt = "%a, %d %b %Y %T GMT";
        // let fmt = "%Y%m%dT%H%M%SZ";
        now.format(fmt).to_string()
    }

    fn iso8601_date_format(&self,now:DateTime<Utc>) -> String {
        let fmt = "%Y%m%d";
        now.format(fmt).to_string()
    }

    fn datetime(&self,now:DateTime<Utc> ) -> String {
        let fmt = "%Y%m%dT%H%M%SZ";
        now.format(fmt).to_string()
    }

    pub fn sign(
        &self,
        bucket: Option<&'a str>,
        key: Option<&'a str>,
        additional_headers: &[String],
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let date = self
            .headers
            .get(DATE)
            .map(|d| d.to_str().unwrap_or_default())
            .unwrap_or_default();
        // let content_type = self.headers
        //     .get(CONTENT_TYPE)
        //     .and_then(|c| Some(c.to_str().unwrap_or_default()))
        //     .unwrap_or_default();
        // let content_md5 = self.headers
        //     .get("Content-MD5")
        //     .and_then(|md5| Some(BASE64_STANDARD.encode(md5.to_str().unwrap_or_default())))
        //     .unwrap_or_default();

        // Credentials information
        // OSS_STS_SECURITY_TOKEN

        // Other Headers
        headers.insert(
            OSS_CONTENT_SHA256,
            HeaderValue::from_static(UNSIGNED_PAYLOAD),
        );
        // Scope
        let product = "oss";
        let now = Utc::now();
        let date = &self.iso8601_date_format(now);
        let scope = format!("{}/{}/{}/{}", date, self.region, product, "aliyun_v4_request");

        let additional_headers = get_common_additional_headers(&headers, additional_headers);

        // CanonicalRequest
        let canonical_request = self.calc_canonical_request(&additional_headers, bucket, key);
        // StringToSign
        let string_to_sign = self.calc_string_to_sign(&now.format("%a, %d %b %Y %T GMT").to_string(), &scope, &canonical_request);
        let signature = self.calc_signature(date, &self.region, product, &string_to_sign);
        let mut authorization = String::from("OSS4-HMAC-SHA256 Credential=");
        authorization.push_str(&self.access_key);
        authorization.push('/');
        authorization.push_str(&scope);
        if !additional_headers.is_empty() {
            authorization.push_str(",AdditionalHeaders=");
            authorization.push_str(additional_headers.join(";").as_str());
        }
        authorization.push_str(",Signature=");
        authorization.push_str(&signature);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&authorization).unwrap(),
        );
        // dbg!(&authorization);
        headers.insert(reqwest::header::DATE, self.date(now).parse().unwrap());
        headers
    }

    fn calc_canonical_request(
        &self,
        additional_headers: &[String],
        bucket: Option<&str>,
        key: Option<&str>,
    ) -> String {
        /*
            Canonical Request
            HTTP Verb + "\n" +
            Canonical URI + "\n" +
            Canonical Query String + "\n" +
            Canonical Headers + "\n" +
            Additional Headers + "\n" +
            Hashed PayLoad
        */
        let mut uri = String::from("/");
        if let Some(bucket) = bucket {
            uri.push_str(bucket);
            uri.push('/');
        }
        if let Some(key) = key {
            uri.push_str(key);
        }

        let canonical_uri = escape_path(&uri,false);
        let canonical_query_string = canonical_query(&self.raw_query);

        //Canonical Headers
        let mut headers_signed = vec![];
        let add_header_map: HashSet<_> = additional_headers
            .iter()
            .map(|x| x.to_lowercase())
            .collect();
        for (k, _v) in self.headers {
            let low_k = k.as_str().to_lowercase();
            if is_default_signed_header(&low_k) {
                headers_signed.push(low_k.clone());
            } else if add_header_map.contains(&low_k) {
                headers_signed.push(low_k);
            }
        }

        headers_signed.sort_unstable();

        let mut canonical_headers = String::new();

        for k in &headers_signed {
            canonical_headers.push_str(k);
            let header_values: Vec<String> = self
                .headers
                .get_all(k)
                .iter()
                .map(|v| v.to_str().unwrap_or_default().into())
                .collect();
       
            canonical_headers.push(':');
            canonical_headers.push_str(header_values.join(",").as_str());
            canonical_headers.push('\n');
        }
        let canonical_additional_headers = additional_headers.join(";");

        let mut hash_payload = "UNSIGNED-PAYLOAD";
        if let Some(val) = self.headers.get(OSS_CONTENT_SHA256) {
            let val = val.to_str().unwrap_or_default();
            if !val.is_empty() {
                hash_payload = val;
            }
        }

        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.method,
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            canonical_additional_headers,
            hash_payload
        )
    }


    /// 计算签名字符串。
    ///
    /// # 参数
    /// * `datetime`: 时间戳。
    /// * `scope`: 作用域。
    /// * `canonical_request`: 规范化请求字符串。
    ///
    /// # 返回
    /// * 签名字符串。
    pub fn calc_string_to_sign(
        &self,
        datetime: &str,
        scope: &str,
        canonical_request: &str,
    ) -> String {
        //StringToSign
        // "OSS4-HMAC-SHA256" + "\n" +
        // TimeStamp + "\n" +
        // Scope + "\n" +
        // Hex(SHA256Hash(Canonical Request))

        // Calculate the SHA256 hash of the canonical request
        let mut hasher = Sha256::new();
        hasher.update(canonical_request.as_bytes());
        let result = hasher.finalize();

        // Construct the string to sign
        format!("OSS4-HMAC-SHA256\n{}\n{}\n{:x}", datetime, scope, result)
    }

    /// 构建作用域字符串。
    ///
    /// # 参数
    /// * `date`: 日期字符串。
    /// * `region`: 区域字符串。
    /// * `product`: 产品字符串。
    ///
    /// # 返回
    /// * 作用域字符串。
    pub fn build_scope(date: &str, region: &str, product: &str) -> String {
        format!("{}/{}/{}/aliyun_v4_request", date, region, product)
    }
    pub fn calc_signature(
        &self,
        date: &str,
        region: &str,
        product: &str,
        string_to_sign: &str,
    ) -> String {
        let h1_key = {
            let mut mac = HmacSha256::new_from_slice(
                (ALIYUN_V4_PREFIX.to_owned() + &self.access_key_secret).as_bytes(),
            )
            .expect("HMAC can take key of any size");
            mac.update(date.as_bytes());
            mac.finalize().into_bytes()
        };

        let h2_key = {
            let mut mac =
                HmacSha256::new_from_slice(&h1_key).expect("HMAC can take key of any size");
            mac.update(region.as_bytes());
            mac.finalize().into_bytes()
        };

        let h3_key = {
            let mut mac =
                HmacSha256::new_from_slice(&h2_key).expect("HMAC can take key of any size");
            mac.update(product.as_bytes());
            mac.finalize().into_bytes()
        };

        let h4_key = {
            let mut mac =
                HmacSha256::new_from_slice(&h3_key).expect("HMAC can take key of any size");
            mac.update(ALIYUN_V4_REQUEST.as_bytes());
            mac.finalize().into_bytes()
        };

        let signature = {
            let mut mac =
                HmacSha256::new_from_slice(&h4_key).expect("HMAC can take key of any size");
            mac.update(string_to_sign.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        };

        signature
    }
}

#[cfg(test)]
mod test {
    use reqwest::Method;

    use super::*;

    #[test]
    fn test_calc_canonical_request() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", HeaderValue::from_static("20231203T121212Z"));
        headers.insert("x-oss-meta-author", HeaderValue::from_static("alice"));
        headers.insert("x-oss-meta-magic", HeaderValue::from_static("abracadabra"));
        headers.insert("x-oss-content-sha256", HeaderValue::from_static("UNSIGNED-PAYLOAD"));
        let v4 = SignerV4::new(
            &headers,
            RequestType::Put,
            "access_key".into(),
            "accesskeysecret".into(),
            "".into(),
            "cn-hangzhou".into(),
        );
        let additional_headers:Vec<String> = vec![];
        let r = v4.calc_canonical_request(&additional_headers, Some("examplebucket"), Some("exampleobject"));
        dbg!(r);
    }
    #[test]
    fn test_calc_string_to_sign() {
        let date = "20231203T121212Z";
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", HeaderValue::from_static("20231203T121212Z"));
        headers.insert("x-oss-meta-author", HeaderValue::from_static("alice"));
        headers.insert("x-oss-meta-magic", HeaderValue::from_static("abracadabra"));
        headers.insert("x-oss-content-sha256", HeaderValue::from_static("UNSIGNED-PAYLOAD"));
        let v4 = SignerV4::new(
            &headers,
            RequestType::Put,
            "access_key".into(),
            "accesskeysecret".into(),
            "".into(),
            "cn-hangzhou".into(),
        );
        let additional_headers:Vec<String> = vec![];
        let r = v4.calc_canonical_request(&additional_headers, Some("examplebucket"), Some("exampleobject"));

        let product = "oss";
        let scope = format!("{}/{}/{}/{}", "20231203", "cn-hangzhou", product, "aliyun_v4_request");
        let sign_str = v4.calc_string_to_sign(date, &scope, &r);
        // OSS4-HMAC-SHA256\n20231203T121212Z\n20231203/cn-hangzhou/oss/aliyun_v4_request\n54848ca3d2c15493f868393c2bfc2040021447187e3c0c0a5712bbefdd03fa84
        // OSS4-HMAC-SHA256\n20231203T121212Z\n20231203/cn-hangzhou/oss/aliyun_v4_request\n129b14df88496f434606e999e35dee010ea1cecfd3ddc378e5ed4989609c1db3
        dbg!(sign_str);
    }

    #[test]
    pub fn test_sign() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", HeaderValue::from_static("20231203T121212Z"));
        headers.insert("x-oss-meta-author", HeaderValue::from_static("alice"));
        headers.insert("x-oss-meta-magic", HeaderValue::from_static("abracadabra"));
        headers.insert("x-oss-content-sha256", HeaderValue::from_static("UNSIGNED-PAYLOAD"));
        let v4 = SignerV4::new(
            &headers,
            RequestType::Put,
            "access_key".into(),
            "accesskeysecret".into(),
            "".into(),
            "cn-hangzhou".into(),
        );
        let s = v4.sign(Some("examplebucket"), Some("exampleobject"), &[]);
        dbg!(s);
    }
}
