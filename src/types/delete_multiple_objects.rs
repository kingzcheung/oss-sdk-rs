//! DeleteMultipleObjects 操作 Input/Output 类型定义

use serde::{Deserialize, Serialize};

/// DeleteMultipleObjects 操作输入
#[derive(Debug, Clone, Default)]
pub struct DeleteMultipleObjectsInput {
    /// Bucket 名称
    pub bucket: String,
    /// 要删除的 Object 列表
    pub objects: Vec<ObjectIdentifier>,
    /// 是否开启简单响应模式
    /// true: OSS 不返回消息体
    /// false: OSS 返回消息体中包含所有删除 Object 的结果（默认）
    pub quiet: bool,
    /// 编码类型
    /// 如果 Key 中包含 XML 1.0 标准不支持的控制字符，可指定 Encoding-type 为 url2
    pub encoding_type: Option<String>,
}

impl DeleteMultipleObjectsInput {
    /// 创建新的 DeleteMultipleObjectsInput 构建器
    pub fn builder() -> DeleteMultipleObjectsInputBuilder {
        DeleteMultipleObjectsInputBuilder::default()
    }
}

/// DeleteMultipleObjectsInput 构建器
#[derive(Debug, Default)]
pub struct DeleteMultipleObjectsInputBuilder {
    bucket: Option<String>,
    objects: Vec<ObjectIdentifier>,
    quiet: bool,
    encoding_type: Option<String>,
}

impl DeleteMultipleObjectsInputBuilder {
    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 添加要删除的 Object
    pub fn object(mut self, key: impl Into<String>) -> Self {
        self.objects.push(ObjectIdentifier {
            key: key.into(),
            version_id: None,
        });
        self
    }

    /// 添加要删除的 Object（带版本 ID）
    pub fn object_with_version(
        mut self,
        key: impl Into<String>,
        version_id: impl Into<String>,
    ) -> Self {
        self.objects.push(ObjectIdentifier {
            key: key.into(),
            version_id: Some(version_id.into()),
        });
        self
    }

    /// 设置要删除的 Object 列表
    pub fn objects(mut self, objects: Vec<ObjectIdentifier>) -> Self {
        self.objects = objects;
        self
    }

    /// 设置是否开启简单响应模式
    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    /// 设置编码类型
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.encoding_type = Some(encoding_type.into());
        self
    }

    /// 构建 DeleteMultipleObjectsInput
    pub fn build(self) -> Result<DeleteMultipleObjectsInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;

        // 最多允许删除 1000 个文件
        if self.objects.len() > 1000 {
            return Err("maximum 1000 objects allowed per request");
        }

        Ok(DeleteMultipleObjectsInput {
            bucket,
            objects: self.objects,
            quiet: self.quiet,
            encoding_type: self.encoding_type,
        })
    }
}

/// Object 标识符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectIdentifier {
    /// Object 名称
    #[serde(rename = "Key")]
    pub key: String,
    /// Object 版本 ID（可选）
    #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl ObjectIdentifier {
    /// 创建新的 ObjectIdentifier
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
        }
    }

    /// 创建带版本 ID 的 ObjectIdentifier
    pub fn with_version(key: impl Into<String>, version_id: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: Some(version_id.into()),
        }
    }
}

/// 删除请求 XML 结构
#[derive(Debug, Serialize)]
#[serde(rename = "Delete")]
struct DeleteRequest {
    #[serde(rename = "Quiet", skip_serializing_if = "Option::is_none")]
    quiet: Option<bool>,
    #[serde(rename = "Object")]
    objects: Vec<ObjectIdentifier>,
    #[serde(rename = "EncodingType", skip_serializing_if = "Option::is_none")]
    encoding_type: Option<String>,
}

/// 将输入转换为 XML 请求体
pub fn to_xml(input: &DeleteMultipleObjectsInput) -> Result<String, quick_xml::DeError> {
    let request = DeleteRequest {
        quiet: if input.quiet { Some(true) } else { None },
        objects: input.objects.clone(),
        encoding_type: input.encoding_type.clone(),
    };

    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    let serialized = quick_xml::se::to_string(&request)?;
    xml.push_str(&serialized);
    Ok(xml)
}

/// DeleteMultipleObjects 操作输出
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename = "DeleteResult")]
pub struct DeleteMultipleObjectsOutput {
    /// 成功删除的 Object 列表
    #[serde(rename = "Deleted", default)]
    pub deleted: Vec<DeletedObject>,
    /// 编码类型
    #[serde(rename = "EncodingType", skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 请求 ID
    #[serde(skip)]
    pub request_id: Option<String>,
}

/// 已删除的 Object 信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeletedObject {
    /// Object 名称
    #[serde(rename = "Key")]
    pub key: String,
    /// 版本 ID
    #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 是否为删除标记
    #[serde(rename = "DeleteMarker", skip_serializing_if = "Option::is_none")]
    pub delete_marker: Option<bool>,
    /// 删除标记的版本 ID
    #[serde(
        rename = "DeleteMarkerVersionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_marker_version_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_xml_verbose_mode() {
        let input = DeleteMultipleObjectsInput {
            bucket: "test-bucket".to_string(),
            objects: vec![
                ObjectIdentifier::new("multipart.data"),
                ObjectIdentifier::new("test.jpg"),
            ],
            quiet: false,
            encoding_type: None,
        };

        let xml = to_xml(&input).unwrap();
        assert!(xml.contains("<Delete>"));
        assert!(xml.contains("<Object>"));
        assert!(xml.contains("<Key>multipart.data</Key>"));
        assert!(xml.contains("<Key>test.jpg</Key>"));
        assert!(!xml.contains("<Quiet>"));
    }

    #[test]
    fn test_to_xml_quiet_mode() {
        let input = DeleteMultipleObjectsInput {
            bucket: "test-bucket".to_string(),
            objects: vec![ObjectIdentifier::new("multipart.data")],
            quiet: true,
            encoding_type: None,
        };

        let xml = to_xml(&input).unwrap();
        assert!(xml.contains("<Quiet>true</Quiet>"));
    }

    #[test]
    fn test_to_xml_with_version_id() {
        let input = DeleteMultipleObjectsInput {
            bucket: "test-bucket".to_string(),
            objects: vec![ObjectIdentifier::with_version(
                "multipart.data",
                "123456789",
            )],
            quiet: false,
            encoding_type: None,
        };

        let xml = to_xml(&input).unwrap();
        assert!(xml.contains("<VersionId>123456789</VersionId>"));
    }

    #[test]
    fn test_deserialize_delete_result() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Deleted>
       <Key>multipart.data</Key>
    </Deleted>
    <Deleted>
       <Key>test.jpg</Key>
    </Deleted>
    <Deleted>
       <Key>demo.jpg</Key>
    </Deleted>
</DeleteResult>"#;

        let output: DeleteMultipleObjectsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.deleted.len(), 3);
        assert_eq!(output.deleted[0].key, "multipart.data");
        assert_eq!(output.deleted[1].key, "test.jpg");
        assert_eq!(output.deleted[2].key, "demo.jpg");
    }

    #[test]
    fn test_deserialize_delete_result_with_delete_marker() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult>
    <Deleted>
       <Key>multipart.data</Key>
       <DeleteMarker>true</DeleteMarker>
       <DeleteMarkerVersionId>CAEQMhiBgIDXiaaB0BYiIGQzYmRkZGUxMTM1ZDRjOTZhNjk4YjRjMTAyZjhl****</DeleteMarkerVersionId>
    </Deleted>
    <Deleted>
       <Key>test.jpg</Key>
       <DeleteMarker>true</DeleteMarker>
       <DeleteMarkerVersionId>CAEQMhiBgIDB3aWB0BYiIGUzYTA3YzliMzVmNzRkZGM5NjllYTVlMjYyYWEy****</DeleteMarkerVersionId>
    </Deleted>
</DeleteResult>"#;

        let output: DeleteMultipleObjectsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.deleted.len(), 2);
        assert_eq!(output.deleted[0].key, "multipart.data");
        assert_eq!(output.deleted[0].delete_marker, Some(true));
        assert!(output.deleted[0].delete_marker_version_id.is_some());
    }

    #[test]
    fn test_deserialize_delete_result_with_version_id() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Deleted>
       <Key>multipart.data</Key>
       <VersionId>CAEQNRiBgIDyz.6C0BYiIGQ2NWEwNmVhNTA3ZTQ3MzM5ODliYjM1ZTdjYjA4****</VersionId>
    </Deleted>
</DeleteResult>"#;

        let output: DeleteMultipleObjectsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.deleted.len(), 1);
        assert_eq!(output.deleted[0].key, "multipart.data");
        assert_eq!(
            output.deleted[0].version_id,
            Some("CAEQNRiBgIDyz.6C0BYiIGQ2NWEwNmVhNTA3ZTQ3MzM5ODliYjM1ZTdjYjA4****".to_string())
        );
    }

    #[test]
    fn test_input_builder() {
        let input = DeleteMultipleObjectsInput::builder()
            .bucket("test-bucket")
            .object("file1.txt")
            .object("file2.txt")
            .object_with_version("file3.txt", "v1")
            .quiet(true)
            .build()
            .unwrap();

        assert_eq!(input.bucket, "test-bucket");
        assert_eq!(input.objects.len(), 3);
        assert!(input.quiet);
        assert_eq!(input.objects[2].version_id, Some("v1".to_string()));
    }

    #[test]
    fn test_max_objects_limit() {
        let mut builder = DeleteMultipleObjectsInput::builder().bucket("test-bucket");

        // 添加 1001 个对象
        for i in 0..1001 {
            builder = builder.object(format!("file{}.txt", i));
        }

        let result = builder.build();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "maximum 1000 objects allowed per request"
        );
    }
}
