//! RestoreObject 集成测试

use oss_sdk_rs::{Client, Config, Credentials, Tier};

mod common;

#[tokio::test]
async fn test_restore_object_builder() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试基本构建器可以创建
    let _builder = client
        .restore_object()
        .bucket("test-bucket")
        .key("archive-object.txt")
        .days(7);
}

#[tokio::test]
async fn test_restore_object_builder_with_tier() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试带优先级的构建器可以创建
    let _builder = client
        .restore_object()
        .bucket("test-bucket")
        .key("cold-archive-object.txt")
        .days(30)
        .tier(Tier::Expedited)
        .version_id("version-123");
}

#[tokio::test]
async fn test_restore_object_builder_with_all_tiers() {
    let config = Config::builder()
        .credentials(Credentials::new("test-key", "test-secret"))
        .region("oss-cn-hangzhou")
        .build()
        .unwrap();

    let client = Client::from_config(config).unwrap();

    // 测试所有优先级
    let tiers = [Tier::Expedited, Tier::Standard, Tier::Bulk];

    for tier in tiers {
        let _builder = client
            .restore_object()
            .bucket("test-bucket")
            .key("object.txt")
            .days(7)
            .tier(tier.clone());
    }
}

#[test]
fn test_tier_variants() {
    // 测试 Tier 枚举
    assert_eq!(Tier::Expedited.to_string(), "Expedited");
    assert_eq!(Tier::Standard.to_string(), "Standard");
    assert_eq!(Tier::Bulk.to_string(), "Bulk");
}

#[test]
fn test_tier_from_str() {
    // 测试 Tier 从字符串转换
    assert_eq!(Tier::from("Expedited"), Tier::Expedited);
    assert_eq!(Tier::from("Standard"), Tier::Standard);
    assert_eq!(Tier::from("Bulk"), Tier::Bulk);
    assert_eq!(Tier::from("unknown"), Tier::Standard); // 默认值
}