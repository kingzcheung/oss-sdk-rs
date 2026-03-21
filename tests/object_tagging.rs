//! 对象标签操作单元测试

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    /// 测试 Tagging 结构体
    #[test]
    fn test_tagging_new() {
        let tagging = oss_sdk_rs::types::Tagging::new();
        assert!(tagging.tag_set.is_empty());
    }

    /// 测试 Tag 结构体
    #[test]
    fn test_tag_creation() {
        let tag = oss_sdk_rs::types::ObjectTag::new("key1", "value1");
        assert_eq!(tag.key, "key1");
        assert_eq!(tag.value, "value1");
    }

    /// 测试 Tagging to_map
    #[test]
    fn test_tagging_to_map() {
        let mut tagging = oss_sdk_rs::types::Tagging::new();
        tagging.tag_set.push(oss_sdk_rs::types::ObjectTag {
            key: "env".to_string(),
            value: "production".to_string(),
        });
        tagging.tag_set.push(oss_sdk_rs::types::ObjectTag {
            key: "team".to_string(),
            value: "backend".to_string(),
        });

        let map = tagging.to_map();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("env"), Some(&"production".to_string()));
        assert_eq!(map.get("team"), Some(&"backend".to_string()));
    }

    /// 测试 PutObjectTaggingInput 构建
    #[test]
    fn test_put_object_tagging_input() {
        let mut tagging = oss_sdk_rs::types::PutTagging::new();
        tagging.tag_set.push(oss_sdk_rs::types::PutTag {
            key: "a".to_string(),
            value: "1".to_string(),
        });

        let input =
            oss_sdk_rs::types::PutObjectTaggingInput::new("my-bucket", "my-object.txt", tagging);

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object.txt");
        assert_eq!(input.tagging.tag_set.len(), 1);
        assert!(input.version_id.is_none());
    }

    /// 测试 PutObjectTaggingInput with version_id
    #[test]
    fn test_put_object_tagging_input_with_version() {
        let tagging = oss_sdk_rs::types::PutTagging::new();
        let input =
            oss_sdk_rs::types::PutObjectTaggingInput::new("my-bucket", "my-object.txt", tagging)
                .version_id("v1");

        assert_eq!(input.version_id, Some("v1".to_string()));
    }

    /// 测试 GetObjectTaggingInput 构建
    #[test]
    fn test_get_object_tagging_input() {
        let input = oss_sdk_rs::types::GetObjectTaggingInput::new("my-bucket", "my-object.txt");

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object.txt");
        assert!(input.version_id.is_none());
    }

    /// 测试 GetObjectTaggingInput with version_id
    #[test]
    fn test_get_object_tagging_input_with_version() {
        let input = oss_sdk_rs::types::GetObjectTaggingInput::new("my-bucket", "my-object.txt")
            .version_id("v1");

        assert_eq!(input.version_id, Some("v1".to_string()));
    }

    /// 测试 DeleteObjectTaggingInput 构建
    #[test]
    fn test_delete_object_tagging_input() {
        let input = oss_sdk_rs::types::DeleteObjectTaggingInput::new("my-bucket", "my-object.txt");

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object.txt");
        assert!(input.version_id.is_none());
    }

    /// 测试 DeleteObjectTaggingInput with version_id
    #[test]
    fn test_delete_object_tagging_input_with_version() {
        let input = oss_sdk_rs::types::DeleteObjectTaggingInput::new("my-bucket", "my-object.txt")
            .version_id("v1");

        assert_eq!(input.version_id, Some("v1".to_string()));
    }

    /// 测试 PutObjectTaggingOutput
    #[test]
    fn test_put_object_tagging_output() {
        let output = oss_sdk_rs::types::PutObjectTaggingOutput::new("request-123");
        assert_eq!(output.request_id, "request-123");
        assert!(output.version_id.is_none());
    }

    /// 测试 GetObjectTaggingOutput
    #[test]
    fn test_get_object_tagging_output() {
        let output = oss_sdk_rs::types::GetObjectTaggingOutput::new("request-123");
        assert_eq!(output.request_id, "request-123");
        assert!(output.version_id.is_none());
        assert!(output.tagging.tag_set.is_empty());
    }

    /// 测试 DeleteObjectTaggingOutput
    #[test]
    fn test_delete_object_tagging_output() {
        let output = oss_sdk_rs::types::DeleteObjectTaggingOutput::new("request-123");
        assert_eq!(output.request_id, "request-123");
        assert!(output.version_id.is_none());
    }

    /// 测试 Tagging to_xml
    #[test]
    fn test_tagging_to_xml() {
        let mut tagging = oss_sdk_rs::types::PutTagging::new();
        tagging.tag_set.push(oss_sdk_rs::types::PutTag {
            key: "a".to_string(),
            value: "1".to_string(),
        });
        tagging.tag_set.push(oss_sdk_rs::types::PutTag {
            key: "b".to_string(),
            value: "2".to_string(),
        });

        let xml = tagging.to_xml();
        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<Tagging>"));
        assert!(xml.contains("<TagSet>"));
        assert!(xml.contains("<Key>a</Key>"));
        assert!(xml.contains("<Value>1</Value>"));
        assert!(xml.contains("<Key>b</Key>"));
        assert!(xml.contains("<Value>2</Value>"));
        assert!(xml.contains("</TagSet>"));
        assert!(xml.contains("</Tagging>"));
    }

    /// 测试 Tagging from_map
    #[test]
    fn test_tagging_from_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        let tagging = oss_sdk_rs::types::PutTagging::from_map(map);
        assert_eq!(tagging.tag_set.len(), 2);
    }
}
