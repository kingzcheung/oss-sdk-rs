//! Object ACL 测试

mod common;

use oss_sdk_rs::{GetObjectAclInput, ObjectAclPermission, PutObjectAclInput};

#[test]
fn test_object_acl_permission() {
    // 测试 ObjectAclPermission 枚举
    assert_eq!(ObjectAclPermission::Private.as_str(), "private");
    assert_eq!(ObjectAclPermission::PublicRead.as_str(), "public-read");
    assert_eq!(
        ObjectAclPermission::PublicReadWrite.as_str(),
        "public-read-write"
    );
    assert_eq!(ObjectAclPermission::Default.as_str(), "default");
}

#[test]
fn test_object_acl_permission_from_str() {
    use std::str::FromStr;

    assert_eq!(
        ObjectAclPermission::from_str("private").unwrap(),
        ObjectAclPermission::Private
    );
    assert_eq!(
        ObjectAclPermission::from_str("public-read").unwrap(),
        ObjectAclPermission::PublicRead
    );
    assert_eq!(
        ObjectAclPermission::from_str("public-read-write").unwrap(),
        ObjectAclPermission::PublicReadWrite
    );
    assert_eq!(
        ObjectAclPermission::from_str("default").unwrap(),
        ObjectAclPermission::Default
    );
    assert!(ObjectAclPermission::from_str("invalid").is_err());
}

#[test]
fn test_object_acl_display() {
    assert_eq!(format!("{}", ObjectAclPermission::Private), "private");
    assert_eq!(
        format!("{}", ObjectAclPermission::PublicRead),
        "public-read"
    );
    assert_eq!(
        format!("{}", ObjectAclPermission::PublicReadWrite),
        "public-read-write"
    );
    assert_eq!(format!("{}", ObjectAclPermission::Default), "default");
}

#[test]
fn test_put_object_acl_input_builder() {
    let input = PutObjectAclInput::builder()
        .bucket("test-bucket")
        .key("test-key")
        .acl(ObjectAclPermission::PublicRead)
        .version_id("123456")
        .build()
        .unwrap();

    assert_eq!(input.bucket, "test-bucket");
    assert_eq!(input.key, "test-key");
    assert_eq!(input.acl, ObjectAclPermission::PublicRead);
    assert_eq!(input.version_id, Some("123456".to_string()));
}

#[test]
fn test_put_object_acl_input_builder_required_fields() {
    // 缺少 bucket
    let result = PutObjectAclInput::builder()
        .key("test-key")
        .acl(ObjectAclPermission::Private)
        .build();
    assert!(result.is_err());

    // 缺少 key
    let result = PutObjectAclInput::builder()
        .bucket("test-bucket")
        .acl(ObjectAclPermission::Private)
        .build();
    assert!(result.is_err());

    // 缺少 acl
    let result = PutObjectAclInput::builder()
        .bucket("test-bucket")
        .key("test-key")
        .build();
    assert!(result.is_err());
}

#[test]
fn test_get_object_acl_input_builder() {
    let input = GetObjectAclInput::builder()
        .bucket("test-bucket")
        .key("test-key")
        .version_id("123456")
        .build()
        .unwrap();

    assert_eq!(input.bucket, "test-bucket");
    assert_eq!(input.key, "test-key");
    assert_eq!(input.version_id, Some("123456".to_string()));
}

#[test]
fn test_get_object_acl_input_builder_required_fields() {
    // 缺少 bucket
    let result = GetObjectAclInput::builder().key("test-key").build();
    assert!(result.is_err());

    // 缺少 key
    let result = GetObjectAclInput::builder().bucket("test-bucket").build();
    assert!(result.is_err());
}

#[test]
fn test_access_control_policy_parse() {
    use oss_sdk_rs::types::AccessControlPolicy;

    let xml = r#"<?xml version="1.0" ?>
<AccessControlPolicy>
    <Owner>
        <ID>0022012****</ID>
        <DisplayName>0022012****</DisplayName>
    </Owner>
    <AccessControlList>
        <Grant>public-read</Grant>
    </AccessControlList>
</AccessControlPolicy>"#;

    let policy = AccessControlPolicy::parse(xml).unwrap();
    assert_eq!(policy.owner.id, "0022012****");
    assert_eq!(policy.owner.display_name, "0022012****");
    assert_eq!(policy.access_control_list.grant, "public-read");
}
