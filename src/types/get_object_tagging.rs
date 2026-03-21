//! GetObjectTagging 操作的类型定义
//!
//! 用于获取对象（Object）的标签（Tagging）信息

use std::collections::HashMap;

/// GetObjectTagging 输入
#[derive(Debug, Clone)]
pub struct GetObjectTaggingInput {
    /// Bucket 名称
    pub bucket: String,
    /// Object 名称
    pub key: String,
    /// 版本 ID
    pub version_id: Option<String>,
}

impl GetObjectTaggingInput {
    /// 创建新的 GetObjectTagging 输入
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }
}

/// GetObjectTagging 输入构建器
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct GetObjectTaggingInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
}

#[allow(dead_code)]
impl GetObjectTaggingInputBuilder {
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

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 构建输入
    pub fn build(self) -> Result<GetObjectTaggingInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;

        Ok(GetObjectTaggingInput {
            bucket,
            key,
            version_id: self.version_id,
        })
    }
}

/// GetObjectTagging 输出
#[derive(Debug, Clone)]
pub struct GetObjectTaggingOutput {
    /// 请求 ID
    pub request_id: String,
    /// 版本 ID
    pub version_id: Option<String>,
    /// 标签集合
    pub tagging: Tagging,
}

impl GetObjectTaggingOutput {
    /// 创建新的输出
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            version_id: None,
            tagging: Tagging::new(),
        }
    }

    /// 设置版本 ID
    pub fn with_version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 设置标签集合
    pub fn with_tagging(mut self, tagging: Tagging) -> Self {
        self.tagging = tagging;
        self
    }

    /// 获取标签 Map
    pub fn tags(&self) -> HashMap<String, String> {
        self.tagging.to_map()
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

    /// 从 XML 解析
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut tagging = Self::new();

        // 简单的 XML 解析
        // 查找所有 <Tag> 元素
        let mut pos = 0;

        while let Some(start) = xml[pos..].find("<Tag>") {
            let tag_content_start = pos + start + 5;
            if let Some(end) = xml[tag_content_start..].find("</Tag>") {
                let tag_content = &xml[tag_content_start..tag_content_start + end];

                // 解析 Key
                let key = if let Some(key_start) = tag_content.find("<Key>") {
                    if let Some(key_end) = tag_content.find("</Key>") {
                        let key = &tag_content[key_start + 5..key_end];
                        Some(unescape_xml(key))
                    } else {
                        None
                    }
                } else {
                    None
                };

                // 解析 Value
                let value = if let Some(value_start) = tag_content.find("<Value>") {
                    if let Some(value_end) = tag_content.find("</Value>") {
                        let value = &tag_content[value_start + 7..value_end];
                        Some(unescape_xml(value))
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let (Some(k), Some(v)) = (key, value) {
                    tagging.tag_set.push(Tag { key: k, value: v });
                }

                pos = tag_content_start + end + 6;
            } else {
                break;
            }
        }

        Ok(tagging)
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

/// XML 反转义
fn unescape_xml(s: &str) -> String {
    s.replace("\u{0026}amp;", "&")
        .replace("\u{0026}lt;", "<")
        .replace("\u{0026}gt;", ">")
        .replace("\u{0026}quot;", "\"")
        .replace("\u{0026}apos;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_object_tagging_input_builder() {
        let input = GetObjectTaggingInputBuilder::default()
            .bucket("my-bucket")
            .key("my-object")
            .version_id("v1")
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object");
        assert_eq!(input.version_id, Some("v1".to_string()));
    }

    #[test]
    fn test_tagging_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Tagging>
  <TagSet>
    <Tag>
      <Key>a</Key>
      <Value>1</Value>
    </Tag>
    <Tag>
      <Key>b</Key>
      <Value>2</Value>
    </Tag>
  </TagSet>
</Tagging>"#;

        let tagging = Tagging::from_xml(xml).unwrap();
        assert_eq!(tagging.tag_set.len(), 2);
        assert_eq!(tagging.tag_set[0].key, "a");
        assert_eq!(tagging.tag_set[0].value, "1");
        assert_eq!(tagging.tag_set[1].key, "b");
        assert_eq!(tagging.tag_set[1].value, "2");
    }

    #[test]
    fn test_tagging_to_map() {
        let tagging = Tagging::new()
            .add_tag("key1", "value1")
            .add_tag("key2", "value2");

        let map = tagging.to_map();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key1"), Some(&"value1".to_string()));
        assert_eq!(map.get("key2"), Some(&"value2".to_string()));
    }
}
