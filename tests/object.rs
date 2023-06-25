//! Copyright The iFREEGROUP/oss-rust-sdk Authors
mod common;
use std::{collections::HashMap};
use oss_rust_sdk::{errors::OSSError, object::ObjectAPI};
use common::*;
#[tokio::test]
async fn test_put_object() -> Result<(), OSSError> {
    let buffer = "test async put object from buffer";
    let oss_instance = create_oss_client();
    let mut headers = HashMap::new();
    headers.insert("content-type", "text/plain");

    let mut oss_sub_resource: HashMap<&str, Option<&str>> = HashMap::new();
    oss_sub_resource.insert("acl", None);
    oss_sub_resource.insert("response-content-type", Some("ContentType"));

    oss_instance
        .put_object(
            buffer.as_bytes(),
            "test/put_object.txt",
            headers,
            oss_sub_resource,
        )
        .await?;

    Ok(())
}


#[tokio::test]
async fn test_append_object() -> Result<(), OSSError> {
    let buffer = "test async put object from buffer";
    let oss_instance = create_oss_client();
    let mut headers = HashMap::new();
    headers.insert("content-type", "text/plain");

    let mut resource:HashMap<&str, Option<&str>>= HashMap::new();
    resource.insert("append", None);
    resource.insert("position", Some("0"));

    let size = oss_instance
        .append_object(
            buffer.as_bytes(),
            "test/your_object_name.txt",
            headers.clone(),
            resource,
        )
        .await?;

    assert!(size.is_some());
    let next_position = format!("{}",size.unwrap());
    let mut resource = HashMap::new();
    resource.insert("append", None);
    resource.insert("position", Some(next_position.as_ref()));
    oss_instance.append_object("buf".as_bytes(), "test/your_object_name.txt", headers, resource).await?;

    Ok(())
}

#[tokio::test]
async fn test_list_objects()->Result<(), OSSError> {
    let oss = create_oss_client();

    let res = oss.list_object( None::<HashMap<&str, &str>>, None).await?;

    dbg!(res);
    Ok(())
}