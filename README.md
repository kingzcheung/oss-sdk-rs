# OSS-Rust-SDK

[![Latest Version](https://img.shields.io/crates/v/oss-sdk-rs.svg)](https://crates.io/crates/oss-sdk-rs)

> Fork from https://github.com/NoXF/oss-rust-sdk

It's an unofficial Rust port for https://github.com/aliyun/aliyun-oss-cpp-sdk, just implement core APIs of https://help.aliyun.com/document_detail/31977.html, everyone is welcome to submit a PR to implement which interface you need.

# Getting Started

## Put Object
```rust
async fn main() -> Result<(), OSSError> {
    let buffer = "test async put object from buffer";
    dotenvy::dotenv().unwrap();

    let buffer = "test async put object from buffer";
    let key_id = env::var("OSS_AK").unwrap();
    let key_secret = env::var("OSS_SK").unwrap();
    let endpoint = env::var("ENDPOINT").unwrap();
    let bucket = env::var("BUCKET").unwrap();
    let oss = OSS::new(
        key_id,
        key_secret,
        endpoint,
        bucket,
    );
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
```
- More example: [examples](https://github.com/iFREEGROUP/oss-sdk-rs/tree/master/examples)

Note: I've removed the blocking code for the repository. If you need it, please move here: [NoXF/oss-rust-sdk](https://github.com/NoXF/oss-rust-sdk)

## License

- Apache License 2.0.
