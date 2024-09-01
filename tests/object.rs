//! Copyright The iFREEGROUP/oss-sdk-rs Authors
mod common;
use common::*;
use oss_sdk_rs::errors::OSSError;

// #[tokio::test]
// async fn test_put_object() -> Result<(), OSSError> {
//     let buffer = "test async put object from buffer";
//     let object_name = "test/put_object.txt";
//     let oss_instance = create_oss_client();
//     let mut headers = HashMap::new();
//     headers.insert("content-type", "text/plain");

//     oss_instance
//         .put_object(buffer.as_bytes(), object_name, headers, None)
//         .await?;
//     println!("上传成功");
//     oss_instance.delete_object(object_name).await?;
//     Ok(())
// }

// #[tokio::test]
// async fn test_append_object() -> Result<(), OSSError> {
//     let buffer = "woshi科楼可乐";
//     let object_name = "test/summary.csv";

//     let oss_instance = create_oss_client();
//     let mut headers = HashMap::new();
//     headers.insert("content-type", "text/plain");

//     let mut resource: HashMap<&str, Option<&str>> = HashMap::new();
//     resource.insert("append", None);
//     resource.insert("position", Some("0"));

//     let size = oss_instance
//         .append_object(buffer.as_bytes(), object_name, headers.clone(), resource)
//         .await?;

//     assert!(size.is_some());

//     // let object = oss_instance.get_object(object_name, headers.clone(), None).await?;
//     // assert_eq!(size.unwrap(), object.len() as u64);

//     let next_position = format!("{}", size.unwrap());

//     dbg!(&next_position);
//     let mut resource = HashMap::new();
//     resource.insert("append", None);
//     resource.insert("position", Some(next_position.as_ref()));
//     oss_instance
//         .append_object("buf".as_bytes(), object_name, headers, resource)
//         .await?;

//     oss_instance.delete_object(object_name).await?;

//     Ok(())
// }

// #[tokio::test]
// async fn test_list_objects() -> Result<(), OSSError> {
//     let oss = create_oss_client();

//     let mut resource: HashMap<&str, Option<&str>> = HashMap::new();
//     resource.insert("prefix", Some("dataset/raw/anitnet_test/empty"));
//     let res = oss.list_object(None::<HashMap<&str, &str>>, resource).await?;

//     dbg!(res);
//     Ok(())
// }

// #[tokio::test]
// async fn test_list_objects_v2() -> Result<(), OSSError> {
//     let oss = create_oss_client();

//     let mut resource: HashMap<&str, Option<&str>> = HashMap::new();
//     resource.insert("prefix", Some("dataset/raw/anitnet_test/empty"));
//     let res = oss.list_object_v2(Some("dataset/raw/anitnet_test/empty"), None).await?;

//     dbg!(res);
//     Ok(())
// }

#[tokio::test]
async fn test_get_object() -> Result<(), OSSError> {
    let oss = create_oss_client();
    let object_name = "test/1665375734.jpg";

// GET /oss.jpg HTTP/1.1
// Host: oss-example.oss-cn-hangzhou.aliyuncs.com
// Date: Tue, 20 Dec 2022 08:48:18 GMT
// Authorization: OSS4-HMAC-SHA256 Credential=AK**ID/20221220/us-east-1/oss/aliyun_v4_request,AdditionalHeaders=host,Signature=18**0a
// x-oss-content-sha256: UNSIGNED-PAYLOAD

    let r = oss.get_object().bucket("tris").key(object_name).send().await?;
    let _ = std::fs::write("result.jpg", r.body.as_slice());
    Ok(())
}

// #[tokio::test]
// async fn test_head_object() ->Result<(), OSSError> {
//     let oss = create_oss_client();
//     let buffer = "test async put object from buffer";
//     let object_name = "test/put_object.txt";
//     let mut headers = HashMap::new();
//     headers.insert("content-type", "text/plain");

//     oss.put_object(buffer.as_bytes(), object_name, headers, None)
//         .await?;

//     let meta = oss.head_object(object_name).await?;
//     assert!(meta.size > 0);
//     oss.delete_object(object_name).await?;
//     Ok(())
// }