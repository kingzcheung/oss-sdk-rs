//! PutObjectTagging 操作的类型定义
//!
//! 用于设置或更新对象（Object）的标签（Tagging）信息

use std::collections::HashMap;

/// PutObjectTagging 输入
#[derive(Debug, Clone)]
pub struct PutObjectTaggingInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// 标签集合
    pub tagging: Tagging,
    /// 版本 ID
    pub version_id: Option<String>,
}

impl PutObjectTaggingInput {
    /// 创建新的 PutObjectTagging 输入
    pub fn new(bucket: impl Into<String>, key: impl Into<String>, tagging: Tagging) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            tagging,
            version_id: None,
        }
    }

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// PutObjectTagging 输出的构建器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PutObjectTaggingInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    tagging: Option<Tagging>,
    version_id: Option<String>,
}

impl Default for PutObjectTaggingInputBuilder {
    fn default() -> Self {
        Self {
            bucket: None,
            key: None,
            tagging: None,
            version_id: None,
        }
    }
}

#[allow(dead_code)]
impl PutObjectTaggingInputBuilder {
    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 设置 Object 名称
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置标签集合
    pub fn tagging(mut self, tagging: Tagging) -> Self {
        self.tagging = Some(tagging);
        self
    }

    /// 添加单个标签
    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let tagging = self.tagging.get_or_insert_with(Tagging::new);
        tagging.tag_set.push(Tag {
            key: key.into(),
            value: value.into(),
        });
        self
    }

    /// 从 HashMap 设置标签
    pub fn tags(mut self, tags: HashMap<String, String>) -> Self {
        self.tagging = Some(Tagging::from_map(tags));
        self
    }

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 构建输入
    pub fn build(self) -> Result<PutObjectTaggingInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let tagging = self.tagging.ok_or("tagging is required")?;

        Ok(PutObjectTaggingInput {
            bucket,
            key,
            tagging,
            version_id: self.version_id,
        })
    }
}

/// 标签集合
#[derive(Debug, Clone, Default)]
pub struct Tagging {
    /// 标签列表
    pub tag_set: Vec<Tag>,
}

impl Tagging {
    /// 创建空的标签集合
    pub fn new() -> Self {
        Self {
            tag_set: Vec::new(),
        }
    }

    /// 添加标签
    pub fn add_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tag_set.push(Tag {
            key: key.into(),
            value: value.into(),
        });
        self
    }

    /// 从 HashMap 创建标签集合
    pub fn from_map(map: HashMap<String, String>) -> Self {
        let tag_set = map
            .into_iter()
            .map(|(key, value)| Tag { key, value })
            .collect();
        Self { tag_set }
    }

    /// 转换为 HashMap
    pub fn to_map(&self) -> HashMap<String, String> {
        self.tag_set
            .iter()
            .map(|tag| (tag.key.clone(), tag.value.clone()))
            .collect()
    }

    /// 转换为 XML
    pub fn to_xml(&self) -> String {
        let mut xml =
            String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Tagging>\n  <TagSet>\n");
        for tag in &self.tag_set {
            xml.push_str(&format!(
                "    <Tag>\n      <Key>{}</Key>\n      <Value>{}</Value>\n    </Tag>\n",
                escape_xml(&tag.key),
                escape_xml(&tag.value)
            ));
        }
        xml.push_str("  </TagSet>\n</Tagging>");
        xml
    }
}

/// 单个标签
#[derive(Debug, Clone)]
pub struct Tag {
    /// 标签键
    pub key: String,
    /// 标签值
    pub value: String,
}

impl Tag {
    /// 创建新标签
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// XML 转义
fn escape_xml(s: &str) -> String {
    let mut result = s.to_string();
    result = result.replace('&', "\u{0026}amp;");
    result = result.replace('<', "\u{0026}lt;");
    result = result.replace('>', "\u{0026}gt;");
    result = result.replace('"', "\u{0026}quot;");
    result = result.replace('\'', "\u{0026}apos;");
    result
}

/// PutObjectTagging 输出
#[derive(Debug, Clone)]
pub struct PutObjectTaggingOutput {
    /// 请求 ID
    pub request_id: String,
    /// 版本 ID
    pub version_id: Option<String>,
}

impl PutObjectTaggingOutput {
    /// 创建新的输出
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            version_id: None,
        }
    }

    /// 设置版本 ID
    pub fn with_version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagging_new() {
        let tagging = Tagging::new().add_tag("a", "1").add_tag("b", "2");

        assert_eq!(tagging.tag_set.len(), 2);
        assert_eq!(tagging.tag_set[0].key, "a");
        assert_eq!(tagging.tag_set[0].value, "1");
    }

    #[test]
    fn test_tagging_to_xml() {
        let tagging = Tagging::new().add_tag("a", "1").add_tag("b", "2");

        let xml = tagging.to_xml();
        assert!(xml.contains("<Key>a</Key>"));
        assert!(xml.contains("<Value>1</Value>"));
        assert!(xml.contains("<Key>b</Key>"));
        assert!(xml.contains("<Value>2</Value>"));
    }

    #[test]
    fn test_tagging_from_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        let tagging = Tagging::from_map(map);
        assert_eq!(tagging.tag_set.len(), 2);
    }

    #[test]
    fn test_put_object_tagging_input_builder() {
        let input = PutObjectTaggingInputBuilder::default()
            .bucket("my-bucket")
            .key("my-object")
            .tag("env", "production")
            .tag("team", "backend")
            .version_id("v1")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object");
        assert_eq!(input.tagging.tag_set.len(), 2);
        assert_eq!(input.version_id, Some("v1".to_string()));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a&b"), "a&amp;b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
    }
}
