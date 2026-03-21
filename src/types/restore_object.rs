//! RestoreObject Input/Output 类型定义
//!
//! 解冻归档、冷归档、深度冷归档类型的 Object。

/// 解冻优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// 高优先级
    /// - 冷归档：1小时内完成解冻
    /// - 深度冷归档：12小时内完成解冻
    Expedited,
    /// 标准（默认）
    /// - 冷归档：2~5小时内完成解冻
    /// - 深度冷归档：48小时内完成解冻
    Standard,
    /// 批量（仅冷归档支持）
    /// - 冷归档：5~12小时内完成解冻
    Bulk,
}

impl Default for Tier {
    fn default() -> Self {
        Self::Standard
    }
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::Expedited => write!(f, "Expedited"),
            Tier::Standard => write!(f, "Standard"),
            Tier::Bulk => write!(f, "Bulk"),
        }
    }
}

impl From<&str> for Tier {
    fn from(s: &str) -> Self {
        match s {
            "Expedited" => Tier::Expedited,
            "Bulk" => Tier::Bulk,
            _ => Tier::Standard,
        }
    }
}

/// RestoreObject 操作输入
#[derive(Debug, Clone)]
pub struct RestoreObjectInput {
    /// 存储桶名称
    pub bucket: String,
    /// 对象键
    pub key: String,
    /// 版本 ID
    pub version_id: Option<String>,
    /// 解冻天数
    /// - 归档类型：1~7 天
    /// - 冷归档/深度冷归档：1~365 天
    pub days: u32,
    /// 解冻优先级（仅冷归档、深度冷归档有效）
    pub tier: Option<Tier>,
}

impl RestoreObjectInput {
    /// 创建新的 RestoreObjectInput 构建器
    pub fn builder() -> RestoreObjectInputBuilder {
        RestoreObjectInputBuilder::default()
    }
}

/// RestoreObjectInput 构建器
#[derive(Debug, Default)]
pub struct RestoreObjectInputBuilder {
    bucket: Option<String>,
    key: Option<String>,
    version_id: Option<String>,
    days: Option<u32>,
    tier: Option<Tier>,
}

impl RestoreObjectInputBuilder {
    /// 设置存储桶名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 设置对象键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置版本 ID
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 设置解冻天数
    /// - 归档类型：1~7 天
    /// - 冷归档/深度冷归档：1~365 天
    pub fn days(mut self, days: u32) -> Self {
        self.days = Some(days);
        self
    }

    /// 设置解冻优先级（仅冷归档、深度冷归档有效）
    pub fn tier(mut self, tier: Tier) -> Self {
        self.tier = Some(tier);
        self
    }

    /// 构建 RestoreObjectInput
    pub fn build(self) -> Result<RestoreObjectInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        let key = self.key.ok_or("key is required")?;
        let days = self.days.ok_or("days is required")?;

        Ok(RestoreObjectInput {
            bucket,
            key,
            version_id: self.version_id,
            days,
            tier: self.tier,
        })
    }
}

/// RestoreObject 操作输出
#[derive(Debug, Default)]
pub struct RestoreObjectOutput {
    /// 请求 ID
    pub request_id: Option<String>,
    /// 版本 ID（仅解冻指定版本时返回）
    pub version_id: Option<String>,
    /// 解冻优先级（仅冷归档或深度冷归档类型 Object 处于解冻状态时返回）
    pub restore_priority: Option<String>,
    /// 响应状态
    pub status: RestoreStatus,
}

/// 解冻状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreStatus {
    /// 已接受（第一次解冻请求，返回 202）
    Accepted,
    /// 成功（Object 已解冻，再次解冻返回 200）
    Ok,
}

impl Default for RestoreStatus {
    fn default() -> Self {
        Self::Accepted
    }
}

/// 生成 RestoreRequest XML
pub fn to_restore_xml(days: u32, tier: Option<Tier>) -> String {
    match tier {
        Some(t) => format!(
            r#"<RestoreRequest><Days>{}</Days><JobParameters><Tier>{}</Tier></JobParameters></RestoreRequest>"#,
            days, t
        ),
        None => format!(
            r#"<RestoreRequest><Days>{}</Days></RestoreRequest>"#,
            days
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = RestoreObjectInput::builder()
            .bucket("my-bucket")
            .key("my-object.txt")
            .days(7)
            .build()
            .unwrap();

        assert_eq!(input.bucket, "my-bucket");
        assert_eq!(input.key, "my-object.txt");
        assert_eq!(input.days, 7);
        assert_eq!(input.version_id, None);
        assert_eq!(input.tier, None);
    }

    #[test]
    fn test_input_builder_with_tier() {
        let input = RestoreObjectInput::builder()
            .bucket("my-bucket")
            .key("my-object.txt")
            .days(30)
            .tier(Tier::Expedited)
            .version_id("version-id")
            .build()
            .unwrap();

        assert_eq!(input.days, 30);
        assert_eq!(input.tier, Some(Tier::Expedited));
        assert_eq!(input.version_id, Some("version-id".to_string()));
    }

    #[test]
    fn test_tier_display() {
        assert_eq!(Tier::Expedited.to_string(), "Expedited");
        assert_eq!(Tier::Standard.to_string(), "Standard");
        assert_eq!(Tier::Bulk.to_string(), "Bulk");
    }

    #[test]
    fn test_tier_from_str() {
        assert_eq!(Tier::from("Expedited"), Tier::Expedited);
        assert_eq!(Tier::from("Standard"), Tier::Standard);
        assert_eq!(Tier::from("Bulk"), Tier::Bulk);
        assert_eq!(Tier::from("unknown"), Tier::Standard);
    }

    #[test]
    fn test_to_xml_without_tier() {
        let xml = to_restore_xml(7, None);
        assert!(xml.contains("<Days>7</Days>"));
        assert!(!xml.contains("<JobParameters>"));
    }

    #[test]
    fn test_to_xml_with_tier() {
        let xml = to_restore_xml(30, Some(Tier::Expedited));
        assert!(xml.contains("<Days>30</Days>"));
        assert!(xml.contains("<JobParameters>"));
        assert!(xml.contains("<Tier>Expedited</Tier>"));
    }

    #[test]
    fn test_input_builder_missing_bucket() {
        let result = RestoreObjectInput::builder()
            .key("my-object.txt")
            .days(7)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "bucket is required");
    }

    #[test]
    fn test_input_builder_missing_key() {
        let result = RestoreObjectInput::builder()
            .bucket("my-bucket")
            .days(7)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "key is required");
    }

    #[test]
    fn test_input_builder_missing_days() {
        let result = RestoreObjectInput::builder()
            .bucket("my-bucket")
            .key("my-object.txt")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "days is required");
    }

    #[test]
    fn test_output_default() {
        let output = RestoreObjectOutput::default();

        assert_eq!(output.request_id, None);
        assert_eq!(output.version_id, None);
        assert_eq!(output.restore_priority, None);
        assert_eq!(output.status, RestoreStatus::Accepted);
    }
}