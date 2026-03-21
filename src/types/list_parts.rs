//! ListParts 接口的类型定义
//!
//! 列举指定 Upload ID 所属的所有已经上传成功 Part

use serde::Deserialize;

/// ListParts 操作的输入
#[derive(Debug, Clone)]
pub struct ListPartsInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// MultipartUpload 事件的 ID
    pub upload_id: String,
    /// 规定在 OSS 响应中的最大 Part 数目，默认值 1000，最大值 1000
    pub max_parts: Option<u32>,
    /// 指定 List 的起始位置，只有 Part Number 大于该参数的 Part 会被列出
    pub part_number_marker: Option<u32>,
    /// 指定对返回的内容进行编码的类型
    pub encoding_type: Option<String>,
}

impl ListPartsInput {
    /// 创建新的输入
    pub fn new(bucket: impl Into<String>, key: impl Into<String>, upload_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            upload_id: upload_id.into(),
            max_parts: None,
            part_number_marker: None,
            encoding_type: None,
        }
    }
}

/// ListParts 操作的输出
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "ListPartsResult")]
pub struct ListPartsOutput {
    /// Bucket 名称
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// 编码类型
    #[serde(rename = "EncodingType", default)]
    pub encoding_type: Option<String>,
    /// Object 名称
    #[serde(rename = "Key")]
    pub key: String,
    /// Upload 事件 ID
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    /// 本次 List 结果的 Part Number 起始位置
    #[serde(rename = "PartNumberMarker", default)]
    pub part_number_marker: Option<u32>,
    /// 接下来请求的 PartNumberMarker 值
    #[serde(rename = "NextPartNumberMarker", default)]
    pub next_part_number_marker: Option<u32>,
    /// 返回请求中最大的 Part 数目
    #[serde(rename = "MaxParts", default)]
    pub max_parts: u32,
    /// 标明本次返回的 ListParts 结果列表是否被截断
    #[serde(rename = "IsTruncated", default)]
    pub is_truncated: bool,
    /// Part 列表
    #[serde(rename = "Part", default)]
    pub parts: Vec<PartInfo>,
}

/// Part 信息
#[derive(Debug, Clone, Deserialize)]
pub struct PartInfo {
    /// Part 编号
    #[serde(rename = "PartNumber")]
    pub part_number: u32,
    /// Part 上传的时间
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    /// Part 内容的 ETag
    #[serde(rename = "ETag")]
    pub etag: String,
    /// Part 大小
    #[serde(rename = "Size")]
    pub size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_parts_input() {
        let input = ListPartsInput::new("my-bucket", "large-file.zip", "upload-id-123");
        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "large-file.zip");
        assert_eq!(input.upload_id, "upload-id-123");
        assert!(input.max_parts.is_none());
        assert!(input.part_number_marker.is_none());
    }

    #[test]
    fn test_list_parts_input_with_options() {
        let mut input = ListPartsInput::new("my-bucket", "large-file.zip", "upload-id-123");
        input.max_parts = Some(100);
        input.part_number_marker = Some(10);
        input.encoding_type = Some("url".to_string());
        assert_eq!(input.max_parts, Some(100));
        assert_eq!(input.part_number_marker, Some(10));
        assert_eq!(input.encoding_type, Some("url".to_string()));
    }

    #[test]
    fn test_deserialize_list_parts_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListPartsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Bucket>multipart_upload</Bucket>
    <Key>multipart.data</Key>
    <UploadId>0004B999EF5A239BB9138C6227D6****</UploadId>
    <NextPartNumberMarker>5</NextPartNumberMarker>
    <MaxParts>1000</MaxParts>
    <IsTruncated>false</IsTruncated>
    <Part>
        <PartNumber>1</PartNumber>
        <LastModified>2012-02-23T07:01:34.000Z</LastModified>
        <ETag>"3349DC700140D7F86A0784842780****"</ETag>
        <Size>6291456</Size>
    </Part>
    <Part>
        <PartNumber>2</PartNumber>
        <LastModified>2012-02-23T07:01:12.000Z</LastModified>
        <ETag>"3349DC700140D7F86A0784842780****"</ETag>
        <Size>6291456</Size>
    </Part>
    <Part>
        <PartNumber>5</PartNumber>
        <LastModified>2012-02-23T07:02:03.000Z</LastModified>
        <ETag>"7265F4D211B56873A381D321F586****"</ETag>
        <Size>1024</Size>
    </Part>
</ListPartsResult>"#;

        let output: ListPartsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.bucket, "multipart_upload");
        assert_eq!(output.key, "multipart.data");
        assert_eq!(output.upload_id, "0004B999EF5A239BB9138C6227D6****");
        assert_eq!(output.max_parts, 1000);
        assert!(!output.is_truncated);
        assert_eq!(output.parts.len(), 3);
        assert_eq!(output.parts[0].part_number, 1);
        assert_eq!(output.parts[0].etag, "\"3349DC700140D7F86A0784842780****\"");
        assert_eq!(output.parts[0].size, 6291456);
        assert_eq!(output.parts[2].part_number, 5);
        assert_eq!(output.parts[2].size, 1024);
    }

    #[test]
    fn test_deserialize_list_parts_output_truncated() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListPartsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Bucket>multipart_upload</Bucket>
    <Key>multipart.data</Key>
    <UploadId>upload-id-123</UploadId>
    <PartNumberMarker>0</PartNumberMarker>
    <NextPartNumberMarker>100</NextPartNumberMarker>
    <MaxParts>100</MaxParts>
    <IsTruncated>true</IsTruncated>
    <Part>
        <PartNumber>1</PartNumber>
        <LastModified>2012-02-23T07:01:34.000Z</LastModified>
        <ETag>"etag-1"</ETag>
        <Size>1024</Size>
    </Part>
</ListPartsResult>"#;

        let output: ListPartsOutput = quick_xml::de::from_str(xml).unwrap();
        assert!(output.is_truncated);
        assert_eq!(output.part_number_marker, Some(0));
        assert_eq!(output.next_part_number_marker, Some(100));
        assert_eq!(output.max_parts, 100);
    }
}