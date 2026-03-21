//! Symlink 测试

mod common;

use oss_sdk_rs::{GetSymlinkInput, PutSymlinkInput};

#[test]
fn test_put_symlink_input_builder() {
    let input = PutSymlinkInput::builder()
        .bucket("test-bucket")
        .key("link.txt")
        .target("original.txt")
        .forbid_overwrite(true)
        .acl("private")
        .storage_class("Standard")
        .build()
        .unwrap();

    assert_eq!(input.bucket, "test-bucket");
    assert_eq!(input.key, "link.txt");
    assert_eq!(input.target, "original.txt");
    assert_eq!(input.forbid_overwrite, Some(true));
    assert_eq!(input.acl, Some("private".to_string()));
    assert_eq!(input.storage_class, Some("Standard".to_string()));
}

#[test]
fn test_put_symlink_input_builder_required_fields() {
    // 缺少 bucket
    let result = PutSymlinkInput::builder()
        .key("link.txt")
        .target("original.txt")
        .build();
    assert!(result.is_err());

    // 缺少 key
    let result = PutSymlinkInput::builder()
        .bucket("test-bucket")
        .target("original.txt")
        .build();
    assert!(result.is_err());

    // 缺少 target
    let result = PutSymlinkInput::builder()
        .bucket("test-bucket")
        .key("link.txt")
        .build();
    assert!(result.is_err());
}

#[test]
fn test_get_symlink_input_builder() {
    let input = GetSymlinkInput::builder()
        .bucket("test-bucket")
        .key("link.txt")
        .version_id("123456")
        .build()
        .unwrap();

    assert_eq!(input.bucket, "test-bucket");
    assert_eq!(input.key, "link.txt");
    assert_eq!(input.version_id, Some("123456".to_string()));
}

#[test]
fn test_get_symlink_input_builder_required_fields() {
    // 缺少 bucket
    let result = GetSymlinkInput::builder().key("link.txt").build();
    assert!(result.is_err());

    // 缺少 key
    let result = GetSymlinkInput::builder().bucket("test-bucket").build();
    assert!(result.is_err());
}

#[test]
fn test_put_symlink_input_optional_fields() {
    // 不设置可选字段
    let input = PutSymlinkInput::builder()
        .bucket("test-bucket")
        .key("link.txt")
        .target("original.txt")
        .build()
        .unwrap();

    assert_eq!(input.forbid_overwrite, None);
    assert_eq!(input.acl, None);
    assert_eq!(input.storage_class, None);
}

#[test]
fn test_get_symlink_input_optional_fields() {
    // 不设置可选字段
    let input = GetSymlinkInput::builder()
        .bucket("test-bucket")
        .key("link.txt")
        .build()
        .unwrap();

    assert_eq!(input.version_id, None);
}
