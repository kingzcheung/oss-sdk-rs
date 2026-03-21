//! ListMultipartUploads 接口的类型定义
//!
//! 列举所有执行中的 Multipart Upload 事件

use serde::Deserialize;

/// ListMultipartUploads 操作的输入
#[derive(Debug, Clone, Default)]
pub struct ListMultipartUploadsInput {
    /// Bucket 名称
    pub bucket: String,
    /// 用于对 Object 名称进行分组的字符
    pub delimiter: Option<String>,
    /// 限定此次返回 Multipart Upload 事件的最大个数，默认值为 1000，最大值为 1000
    pub max_uploads: Option<u32>,
    /// 与 upload_id_marker 参数配合使用，用于指定返回结果的起始位置
    pub key_marker: Option<String>,
    /// 限定返回的 Object Key 必须以 prefix 作为前缀
    pub prefix: Option<String>,
    /// 与 key_marker 参数配合使用，用于指定返回结果的起始位置
    pub upload_id_marker: Option<String>,
    /// 指定对返回的内容进行编码的类型
    pub encoding_type: Option<String>,
}

impl ListMultipartUploadsInput {
    /// 创建新的输入
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            ..Default::default()
        }
    }
}

/// ListMultipartUploads 操作的输出
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "ListMultipartUploadsResult")]
pub struct ListMultipartUploadsOutput {
    /// Bucket 名称
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// 编码类型
    #[serde(rename = "EncodingType", default)]
    pub encoding_type: Option<String>,
    /// 列表的起始 Object 位置
    #[serde(rename = "KeyMarker", default)]
    pub key_marker: Option<String>,
    /// 列表的起始 UploadId 位置
    #[serde(rename = "UploadIdMarker", default)]
    pub upload_id_marker: Option<String>,
    /// 接下来请求的 KeyMarker 值
    #[serde(rename = "NextKeyMarker", default)]
    pub next_key_marker: Option<String>,
    /// 接下来请求的 UploadMarker 值
    #[serde(rename = "NextUploadIdMarker", default)]
    pub next_upload_id_marker: Option<String>,
    /// 返回的最大 Upload 个数
    #[serde(rename = "MaxUploads", default)]
    pub max_uploads: u32,
    /// 分隔符
    #[serde(rename = "Delimiter", default)]
    pub delimiter: Option<String>,
    /// 前缀
    #[serde(rename = "Prefix", default)]
    pub prefix: Option<String>,
    /// 表示本次返回的结果列表是否被截断
    #[serde(rename = "IsTruncated", default)]
    pub is_truncated: bool,
    /// Multipart Upload 事件列表
    #[serde(rename = "Upload", default)]
    pub uploads: Vec<MultipartUpload>,
    /// 公共前缀列表
    #[serde(rename = "CommonPrefixes", default)]
    pub common_prefixes: Vec<CommonPrefix>,
}

/// Multipart Upload 事件信息
#[derive(Debug, Clone, Deserialize)]
pub struct MultipartUpload {
    /// Object 名称
    #[serde(rename = "Key")]
    pub key: String,
    /// Multipart Upload 事件的 ID
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    /// Multipart Upload 事件初始化的时间
    #[serde(rename = "Initiated")]
    pub initiated: String,
}

/// 公共前缀
#[derive(Debug, Clone, Deserialize)]
pub struct CommonPrefix {
    /// 前缀
    #[serde(rename = "Prefix")]
    pub prefix: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_multipart_uploads_input() {
        let input = ListMultipartUploadsInput::new("my-bucket");
        assert_eq!(input.bucket, "my-bucket");
        assert!(input.delimiter.is_none());
        assert!(input.max_uploads.is_none());
    }

    #[test]
    fn test_list_multipart_uploads_input_with_options() {
        let input = ListMultipartUploadsInput {
            bucket: "my-bucket".to_string(),
            delimiter: Some("/".to_string()),
            max_uploads: Some(100),
            key_marker: Some("key-marker".to_string()),
            prefix: Some("prefix/".to_string()),
            upload_id_marker: Some("upload-id-marker".to_string()),
            encoding_type: Some("url".to_string()),
        };
        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.delimiter, Some("/".to_string()));
        assert_eq!(input.max_uploads, Some(100));
        assert_eq!(input.key_marker, Some("key-marker".to_string()));
        assert_eq!(input.prefix, Some("prefix/".to_string()));
    }

    #[test]
    fn test_deserialize_list_multipart_uploads_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListMultipartUploadsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Bucket>oss-example</Bucket>
    <KeyMarker></KeyMarker>
    <UploadIdMarker></UploadIdMarker>
    <NextKeyMarker>oss.avi</NextKeyMarker>
    <NextUploadIdMarker>0004B99B8E707874FC2D692FA5D77D3F</NextUploadIdMarker>
    <Delimiter></Delimiter>
    <Prefix></Prefix>
    <MaxUploads>1000</MaxUploads>
    <IsTruncated>false</IsTruncated>
    <Upload>
        <Key>multipart.data</Key>
        <UploadId>0004B999EF518A1FE585B0C9360DC4C8</UploadId>
        <Initiated>2012-02-23T04:18:23.000Z</Initiated>
    </Upload>
    <Upload>
        <Key>multipart.data</Key>
        <UploadId>0004B999EF5A239BB9138C6227D6****</UploadId>
        <Initiated>2012-02-23T04:18:23.000Z</Initiated>
    </Upload>
    <Upload>
        <Key>oss.avi</Key>
        <UploadId>0004B99B8E707874FC2D692FA5D7****</UploadId>
        <Initiated>2012-02-23T06:14:27.000Z</Initiated>
    </Upload>
</ListMultipartUploadsResult>"#;

        let output: ListMultipartUploadsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.bucket, "oss-example");
        assert_eq!(output.max_uploads, 1000);
        assert!(!output.is_truncated);
        assert_eq!(output.uploads.len(), 3);
        assert_eq!(output.uploads[0].key, "multipart.data");
        assert_eq!(output.uploads[0].upload_id, "0004B999EF518A1FE585B0C9360DC4C8");
        assert_eq!(output.uploads[0].initiated, "2012-02-23T04:18:23.000Z");
        assert_eq!(output.next_key_marker, Some("oss.avi".to_string()));
    }

    #[test]
    fn test_deserialize_list_multipart_uploads_with_prefix() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListMultipartUploadsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Bucket>oss-example</Bucket>
    <KeyMarker></KeyMarker>
    <UploadIdMarker></UploadIdMarker>
    <NextKeyMarker></NextKeyMarker>
    <NextUploadIdMarker></NextUploadIdMarker>
    <Delimiter>/</Delimiter>
    <Prefix>photos/</Prefix>
    <MaxUploads>1000</MaxUploads>
    <IsTruncated>false</IsTruncated>
    <Upload>
        <Key>photos/test.jpg</Key>
        <UploadId>upload-id-123</UploadId>
        <Initiated>2012-02-23T04:18:23.000Z</Initiated>
    </Upload>
    <CommonPrefixes>
        <Prefix>photos/2021/</Prefix>
    </CommonPrefixes>
</ListMultipartUploadsResult>"#;

        let output: ListMultipartUploadsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.bucket, "oss-example");
        assert_eq!(output.delimiter, Some("/".to_string()));
        assert_eq!(output.prefix, Some("photos/".to_string()));
        assert_eq!(output.uploads.len(), 1);
        assert_eq!(output.common_prefixes.len(), 1);
        assert_eq!(output.common_prefixes[0].prefix, "photos/2021/");
    }
}