//! CompleteMultipartUpload 操作类型定义
//!
//! 完成分片上传，将所有已上传的 Part 合并成一个完整的 Object。

use serde::{Deserialize, Serialize};

/// Part 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartItem {
    /// Part 编号
    #[serde(rename = "PartNumber")]
    pub part_number: u32,
    /// ETag 值
    #[serde(rename = "ETag")]
    pub etag: String,
}

impl PartItem {
    /// 创建新的 PartItem
    pub fn new(part_number: u32, etag: impl Into<String>) -> Self {
        Self {
            part_number,
            etag: etag.into(),
        }
    }
}

/// CompleteMultipartUpload 输入
#[derive(Debug, Clone, Default)]
pub struct CompleteMultipartUploadInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// 分片上传 ID
    pub upload_id: String,
    /// Part 列表
    pub parts: Vec<PartItem>,
    /// 是否禁止覆盖同名 Object
    pub forbid_overwrite: Option<bool>,
    /// 是否自动完成所有 Part
    pub complete_all: bool,
    /// Object ACL
    pub object_acl: Option<String>,
    /// 编码类型
    pub encoding_type: Option<String>,
}

/// Object ACL 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectAcl {
    /// 默认（遵循 Bucket 权限）
    #[default]
    Default,
    /// 私有
    Private,
    /// 公共读
    PublicRead,
    /// 公共读写
    PublicReadWrite,
}

impl std::fmt::Display for ObjectAcl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectAcl::Default => write!(f, "default"),
            ObjectAcl::Private => write!(f, "private"),
            ObjectAcl::PublicRead => write!(f, "public-read"),
            ObjectAcl::PublicReadWrite => write!(f, "public-read-write"),
        }
    }
}

impl From<&str> for ObjectAcl {
    fn from(s: &str) -> Self {
        match s {
            "default" => ObjectAcl::Default,
            "private" => ObjectAcl::Private,
            "public-read" => ObjectAcl::PublicRead,
            "public-read-write" => ObjectAcl::PublicReadWrite,
            _ => ObjectAcl::Default,
        }
    }
}

/// CompleteMultipartUpload 输出
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "CompleteMultipartUploadResult")]
pub struct CompleteMultipartUploadOutput {
    /// Object 的 URL
    #[serde(rename = "Location")]
    pub location: String,
    /// Bucket 名称
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// Object 名称
    #[serde(rename = "Key")]
    pub key: String,
    /// ETag
    #[serde(rename = "ETag")]
    pub etag: String,
    /// 编码类型
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,
}

/// CompleteMultipartUpload 响应
#[derive(Debug, Clone, Default)]
pub struct CompleteMultipartUploadResponse {
    /// 完成结果
    pub result: Option<CompleteMultipartUploadOutput>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// Object 版本 ID
    pub version_id: Option<String>,
    /// 服务端加密算法
    pub server_side_encryption: Option<String>,
}

/// 生成 CompleteMultipartUpload XML
pub fn to_complete_multipart_upload_xml(parts: &[PartItem]) -> String {
    let mut xml = String::from("<CompleteMultipartUpload>");
    for part in parts {
        xml.push_str(&format!(
            "<Part><PartNumber>{}</PartNumber><ETag>{}</ETag></Part>",
            part.part_number, part.etag
        ));
    }
    xml.push_str("</CompleteMultipartUpload>");
    xml
}

/// CompleteMultipartUpload 输入构建器
#[derive(Debug, Default)]
pub struct CompleteMultipartUploadInputBuilder {
    inner: CompleteMultipartUploadInput,
}

impl CompleteMultipartUploadInputBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = key.into();
        self
    }

    /// 设置分片上传 ID
    pub fn upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.inner.upload_id = upload_id.into();
        self
    }

    /// 添加 Part
    pub fn part(mut self, part_number: u32, etag: impl Into<String>) -> Self {
        self.inner.parts.push(PartItem::new(part_number, etag));
        self
    }

    /// 设置 Part 列表
    pub fn parts(mut self, parts: Vec<PartItem>) -> Self {
        self.inner.parts = parts;
        self
    }

    /// 设置是否禁止覆盖同名 Object
    pub fn forbid_overwrite(mut self, forbid_overwrite: bool) -> Self {
        self.inner.forbid_overwrite = Some(forbid_overwrite);
        self
    }

    /// 设置是否自动完成所有 Part
    /// 如果设置为 true，则 OSS 会列举当前 UploadId 已上传的所有 Part
    pub fn complete_all(mut self, complete_all: bool) -> Self {
        self.inner.complete_all = complete_all;
        self
    }

    /// 设置 Object ACL
    pub fn object_acl(mut self, acl: ObjectAcl) -> Self {
        self.inner.object_acl = Some(acl.to_string());
        self
    }

    /// 设置编码类型
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// 构建 Input
    pub fn build(self) -> Result<CompleteMultipartUploadInput, &'static str> {
        if self.inner.bucket.is_empty() {
            return Err("bucket is required");
        }
        if self.inner.key.is_empty() {
            return Err("key is required");
        }
        if self.inner.upload_id.is_empty() {
            return Err("upload_id is required");
        }
        // 如果不是 complete_all，则需要至少一个 Part
        if !self.inner.complete_all && self.inner.parts.is_empty() {
            return Err("parts is required when complete_all is false");
        }
        Ok(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_item() {
        let part = PartItem::new(1, "\"etag-123\"");
        assert_eq!(part.part_number, 1);
        assert_eq!(part.etag, "\"etag-123\"");
    }

    #[test]
    fn test_object_acl_display() {
        assert_eq!(ObjectAcl::Default.to_string(), "default");
        assert_eq!(ObjectAcl::Private.to_string(), "private");
        assert_eq!(ObjectAcl::PublicRead.to_string(), "public-read");
        assert_eq!(ObjectAcl::PublicReadWrite.to_string(), "public-read-write");
    }

    #[test]
    fn test_object_acl_from_str() {
        assert_eq!(ObjectAcl::from("default"), ObjectAcl::Default);
        assert_eq!(ObjectAcl::from("private"), ObjectAcl::Private);
        assert_eq!(ObjectAcl::from("public-read"), ObjectAcl::PublicRead);
        assert_eq!(ObjectAcl::from("public-read-write"), ObjectAcl::PublicReadWrite);
        assert_eq!(ObjectAcl::from("unknown"), ObjectAcl::Default);
    }

    #[test]
    fn test_input_builder() {
        let input = CompleteMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .part(1, "\"etag-1\"")
            .part(2, "\"etag-2\"")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "large-file.zip");
        assert_eq!(input.upload_id, "upload-id");
        assert_eq!(input.parts.len(), 2);
        assert_eq!(input.parts[0].part_number, 1);
        assert_eq!(input.parts[1].part_number, 2);
    }

    #[test]
    fn test_input_builder_with_options() {
        let input = CompleteMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .part(1, "\"etag-1\"")
            .forbid_overwrite(true)
            .object_acl(ObjectAcl::Private)
            .encoding_type("url")
            .build()
            .unwrap();

        assert_eq!(input.forbid_overwrite, Some(true));
        assert_eq!(input.object_acl, Some("private".to_string()));
        assert_eq!(input.encoding_type, Some("url".to_string()));
    }

    #[test]
    fn test_input_builder_complete_all() {
        let input = CompleteMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .complete_all(true)
            .build()
            .unwrap();

        assert!(input.complete_all);
        assert!(input.parts.is_empty());
    }

    #[test]
    fn test_input_builder_missing_bucket() {
        let result = CompleteMultipartUploadInputBuilder::new()
            .key("large-file.zip")
            .upload_id("upload-id")
            .part(1, "\"etag-1\"")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "bucket is required");
    }

    #[test]
    fn test_input_builder_missing_parts() {
        let result = CompleteMultipartUploadInputBuilder::new()
            .bucket("my-bucket")
            .key("large-file.zip")
            .upload_id("upload-id")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "parts is required when complete_all is false");
    }

    #[test]
    fn test_to_xml() {
        let parts = vec![
            PartItem::new(1, "\"etag-1\""),
            PartItem::new(5, "\"etag-5\""),
        ];
        let xml = to_complete_multipart_upload_xml(&parts);

        assert!(xml.contains("<CompleteMultipartUpload>"));
        assert!(xml.contains("<Part><PartNumber>1</PartNumber><ETag>\"etag-1\"</ETag></Part>"));
        assert!(xml.contains("<Part><PartNumber>5</PartNumber><ETag>\"etag-5\"</ETag></Part>"));
        assert!(xml.contains("</CompleteMultipartUpload>"));
    }

    #[test]
    fn test_deserialize_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CompleteMultipartUploadResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Location>http://oss-example.oss-cn-hangzhou.aliyuncs.com/multipart.data</Location>
    <Bucket>oss-example</Bucket>
    <Key>multipart.data</Key>
    <ETag>"B864DB6A936D376F9F8D3ED3BBE540****"</ETag>
    <EncodingType>url</EncodingType>
</CompleteMultipartUploadResult>"#;

        let output: CompleteMultipartUploadOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.location, "http://oss-example.oss-cn-hangzhou.aliyuncs.com/multipart.data");
        assert_eq!(output.bucket, "oss-example");
        assert_eq!(output.key, "multipart.data");
        assert_eq!(output.etag, "\"B864DB6A936D376F9F8D3ED3BBE540****\"");
        assert_eq!(output.encoding_type, Some("url".to_string()));
    }
}