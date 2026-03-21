//! AWS 风格 API 集成测试
//!
//! 运行测试前请确保：
//! 1. 复制 .env.example 为 .env
//! 2. 在 .env 中填入实际的阿里云 OSS 配置信息
//!
//! 环境变量：
//! - OSS_ACCESS_KEY_ID: 阿里云 AccessKey ID
//! - OSS_ACCESS_KEY_SECRET: 阿里云 AccessKey Secret
//! - OSS_ENDPOINT: OSS endpoint (如: https://oss-cn-hangzhou.aliyuncs.com)
//! - OSS_BUCKET: 测试用的 bucket 名称

mod common;
use std::env;

use common::*;
use oss_sdk_rs::errors::OSSError;

/// 测试 put_object API
#[tokio::test]
async fn test_put_object() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let content = "test async put object from buffer - aws style api";
    let object_name = "test/aws_style_put_object.txt";
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    let output = oss
        .put_object()
        .bucket(&bucket)
        .key(object_name)
        .body(content.as_bytes().to_vec())
        .send()
        .await?;

    println!("Put object result - ETag: {:?}", output.etag);
    assert!(output.etag.is_some());
    Ok(())
}

/// 测试 get_object API
#[tokio::test]
async fn test_get_object() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let object_name = "test/aws_style_put_object.txt";

    let output = oss
        .get_object()
        .bucket(&bucket)
        .key(object_name)
        .send()
        .await?;

    let body = output.body.collect().await.map_err(|e| OSSError::Io(e))?;

    let content = String::from_utf8(body.to_vec()).map_err(|e| OSSError::String(e))?;

    println!("Get object content: {}", content);
    assert!(content.contains("test async put object"));
    Ok(())
}

/// 测试 list_objects API
#[tokio::test]
async fn test_list_objects() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    let output = oss
        .list_objects()
        .bucket(&bucket)
        .prefix("test/")
        .max_keys(10)
        .send()
        .await?;

    println!("List objects result:");
    println!("  Prefix: {:?}", output.prefix);
    println!("  Max keys: {:?}", output.max_keys);
    println!("  Is truncated: {:?}", output.is_truncated);

    if let Some(contents) = &output.contents {
        for obj in contents {
            println!("  - Key: {:?}, Size: {:?}", obj.key, obj.size);
        }
    }

    assert!(output.max_keys.is_some());
    Ok(())
}

/// 测试 head_object API
#[tokio::test]
async fn test_head_object() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let object_name = "test/aws_style_put_object.txt";

    let output = oss
        .head_object()
        .bucket(&bucket)
        .key(object_name)
        .send()
        .await?;

    println!("Head object result:");
    println!("  Content length: {:?}", output.content_length);
    println!("  Content type: {:?}", output.content_type);
    println!("  ETag: {:?}", output.etag);
    println!("  Last modified: {:?}", output.last_modified);

    assert!(output.content_length.is_some());
    assert!(output.content_length.unwrap() > 0);
    Ok(())
}

/// 测试 copy_object API
#[tokio::test]
async fn test_copy_object() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let source_key = "test/aws_style_put_object.txt";
    let dest_key = "test/aws_style_copy_object.txt";

    let copy_source = format!("{}/{}", bucket, source_key);

    let output = oss
        .copy_object()
        .bucket(&bucket)
        .key(dest_key)
        .copy_source(&copy_source)
        .send()
        .await?;

    println!("Copy object result:");
    println!("  ETag: {:?}", output.etag);
    println!("  Last modified: {:?}", output.last_modified);

    // 清理复制的对象
    oss.delete_object()
        .bucket(&bucket)
        .key(dest_key)
        .send()
        .await?;

    Ok(())
}

/// 测试 delete_object API
#[tokio::test]
async fn test_delete_object() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    // 先创建一个对象
    let temp_object_name = "test/temp_to_delete.txt";
    oss.put_object()
        .bucket(&bucket)
        .key(temp_object_name)
        .body(b"temporary content".to_vec())
        .send()
        .await?;

    // 然后删除它
    let output = oss
        .delete_object()
        .bucket(&bucket)
        .key(temp_object_name)
        .send()
        .await?;

    println!("Delete object result:");
    println!("  Request ID: {:?}", output.request_id);

    Ok(())
}

/// 完整的生命周期测试：put -> get -> head -> delete
#[tokio::test]
async fn test_object_lifecycle() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let object_name = "test/lifecycle_test.txt";
    let content = b"lifecycle test content".to_vec();

    // 1. Put object
    println!("1. Putting object...");
    let put_output = oss
        .put_object()
        .bucket(&bucket)
        .key(object_name)
        .body(content.clone())
        .content_type("text/plain")
        .send()
        .await?;
    println!("   Put ETag: {:?}", put_output.etag);

    // 2. Get object
    println!("2. Getting object...");
    let get_output = oss
        .get_object()
        .bucket(&bucket)
        .key(object_name)
        .send()
        .await?;
    let body = get_output
        .body
        .collect()
        .await
        .map_err(|e| OSSError::Io(e))?;
    println!("   Get content length: {}", body.len());
    assert_eq!(body.to_vec(), content);

    // 3. Head object
    println!("3. Heading object...");
    let head_output = oss
        .head_object()
        .bucket(&bucket)
        .key(object_name)
        .send()
        .await?;
    println!("   Head content length: {:?}", head_output.content_length);
    assert_eq!(head_output.content_length, Some(content.len() as u64));

    // 4. Delete object
    println!("4. Deleting object...");
    let delete_output = oss
        .delete_object()
        .bucket(&bucket)
        .key(object_name)
        .send()
        .await?;
    println!("   Delete request ID: {:?}", delete_output.request_id);

    // 5. Verify deletion - get should fail
    println!("5. Verifying deletion...");
    let result = oss
        .get_object()
        .bucket(&bucket)
        .key(object_name)
        .send()
        .await;
    assert!(result.is_err(), "Object should be deleted");

    println!("Lifecycle test completed successfully!");
    Ok(())
}

/// 测试带范围请求的 get_object
#[tokio::test]
async fn test_get_object_with_range() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");
    let object_name = "test/range_test.txt";

    // 先上传一个较大的文件
    let content = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec();
    oss.put_object()
        .bucket(&bucket)
        .key(object_name)
        .body(content.clone())
        .send()
        .await?;

    // 读取部分内容
    let output = oss
        .get_object()
        .bucket(&bucket)
        .key(object_name)
        .range("bytes=0-9")
        .send()
        .await?;

    let body = output.body.collect().await.map_err(|e| OSSError::Io(e))?;
    let partial_content = body.to_vec();

    println!(
        "Range request content: {:?}",
        String::from_utf8_lossy(&partial_content)
    );
    assert_eq!(partial_content, b"0123456789");

    // 清理
    oss.delete_object()
        .bucket(&bucket)
        .key(object_name)
        .send()
        .await?;

    Ok(())
}

/// 测试 list_objects 带分隔符
#[tokio::test]
async fn test_list_objects_with_delimiter() -> Result<(), OSSError> {
    dotenvy::dotenv().ok();
    let oss = create_oss_client();
    let bucket = env::var("OSS_BUCKET").expect("OSS_BUCKET must be set in .env");

    // 创建一些测试对象
    let test_files = vec![
        "test/dir1/file1.txt",
        "test/dir1/file2.txt",
        "test/dir2/file3.txt",
    ];

    for file in &test_files {
        oss.put_object()
            .bucket(&bucket)
            .key(*file)
            .body(b"test content".to_vec())
            .send()
            .await?;
    }

    // 使用分隔符列出对象
    let output = oss
        .list_objects()
        .bucket(&bucket)
        .prefix("test/")
        .delimiter("/")
        .max_keys(100)
        .send()
        .await?;

    println!("List with delimiter result:");
    println!("  Prefix: {:?}", output.prefix);
    println!("  Delimiter: {:?}", output.delimiter);

    if let Some(common_prefixes) = &output.common_prefixes {
        println!("  Common prefixes:");
        for prefix in common_prefixes {
            println!("    - {:?}", prefix.prefix);
        }
    }

    if let Some(contents) = &output.contents {
        println!("  Objects:");
        for obj in contents {
            println!("    - {:?}", obj.key);
        }
    }

    // 清理
    for file in &test_files {
        oss.delete_object()
            .bucket(&bucket)
            .key(*file)
            .send()
            .await?;
    }

    Ok(())
}
