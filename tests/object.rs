//! Copyright The iFREEGROUP/oss-sdk-rs Authors
mod common;
use std::{collections::HashMap};
use oss_sdk_rs::{errors::OSSError, object::ObjectAPI};
use common::*;
#[tokio::test]
async fn test_put_object() -> Result<(), OSSError> {
    let buffer = "test async put object from buffer";
    let object_name = "test/put_object.txt";
    let oss_instance = create_oss_client();
    let mut headers = HashMap::new();
    headers.insert("content-type", "text/plain");

    oss_instance
        .put_object(
            buffer.as_bytes(),
            object_name,
            headers,
            None,
        )
        .await?;
    oss_instance.delete_object(object_name).await?;
    Ok(())
}


#[tokio::test]
async fn test_append_object() -> Result<(), OSSError> {
    let buffer = "woshi科楼可乐";
    let object_name = "test/summary.csv";

    let oss_instance = create_oss_client();
    let mut headers = HashMap::new();
    headers.insert("content-type", "text/plain");

    let mut resource:HashMap<&str, Option<&str>>= HashMap::new();
    resource.insert("append", None);
    resource.insert("position", Some("0"));

    let size = oss_instance
        .append_object(
            buffer.as_bytes(),
            object_name,
            headers.clone(),
            resource,
        )
        .await?;

    assert!(size.is_some());
    
    // let object = oss_instance.get_object(object_name, headers.clone(), None).await?;
    // assert_eq!(size.unwrap(), object.len() as u64);

    let next_position = format!("{}",size.unwrap());

    dbg!(&next_position);
    let mut resource = HashMap::new();
    resource.insert("append", None);
    resource.insert("position", Some(next_position.as_ref()));
    oss_instance.append_object("buf".as_bytes(), object_name, headers, resource).await?;
    
    oss_instance.delete_object(object_name).await?;

    Ok(())
}

#[tokio::test]
async fn test_list_objects()->Result<(), OSSError> {
    let oss = create_oss_client();

    let res = oss.list_object( None::<HashMap<&str, &str>>, None).await?;

    dbg!(res);
    Ok(())
}