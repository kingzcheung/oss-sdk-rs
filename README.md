# OSS SDK for Rust

[![Latest Version](https://img.shields.io/crates/v/oss-sdk-rs.svg)](https://crates.io/crates/oss-sdk-rs)
[![Documentation](https://docs.rs/oss-sdk-rs/badge.svg)](https://docs.rs/oss-sdk-rs)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

> Fork from https://github.com/NoXF/oss-rust-sdk

阿里云 OSS（对象存储服务）的非官方 Rust SDK，采用现代风格的 API 设计，提供完整的异步支持。

## 特性

- 🚀 **异步优先** - 基于 Tokio 的完全异步 API
- 🔐 **V4 签名** - 支持阿里云 OSS V4 签名算法
- 🎯 **类型安全** - 强类型 API，编译时捕获错误
- 🛠️ **Builder 模式** - 流畅的 API 设计，易于使用
- 📦 **完整功能** - 支持对象操作、分片上传、Bucket 管理等

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
oss-sdk-rs = "1.2.4"
tokio = { version = "1", features = ["full"] }
```

## 快速开始

### 配置凭证

创建 `.env` 文件或设置环境变量：

```env
OSS_ACCESS_KEY_ID=your_access_key_id
OSS_ACCESS_KEY_SECRET=your_access_key_secret
OSS_ENDPOINT=https://oss-cn-hangzhou.aliyuncs.com
OSS_BUCKET=your-bucket-name
```

### 基本用法

```rust
use std::env;
use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建凭证
    let credentials = Credentials::new(
        env::var("OSS_ACCESS_KEY_ID")?,
        env::var("OSS_ACCESS_KEY_SECRET")?,
    );

    // 创建配置
    let config = Config::builder()
        .endpoint(env::var("OSS_ENDPOINT")?)
        .credentials(credentials)
        .region("cn-hangzhou")
        .build()?;

    // 创建客户端
    let client = Client::from_config(config)?;

    // 上传对象
    let output = client
        .put_object()
        .bucket("my-bucket")
        .key("hello.txt")
        .body(b"Hello, OSS!".to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("上传成功，ETag: {:?}", output.etag);

    Ok(())
}
```

## API 文档

### 对象操作

#### 上传对象 (PutObject)

```rust
let output = client
    .put_object()
    .bucket("my-bucket")
    .key("path/to/file.txt")
    .body(file_content)
    .content_type("text/plain")
    .send()
    .await?;
```

#### 下载对象 (GetObject)

```rust
let output = client
    .get_object()
    .bucket("my-bucket")
    .key("path/to/file.txt")
    .send()
    .await?;

let content = output.body_into_vec().await?;
```

#### 删除对象 (DeleteObject)

```rust
let output = client
    .delete_object()
    .bucket("my-bucket")
    .key("path/to/file.txt")
    .send()
    .await?;
```

#### 复制对象 (CopyObject)

```rust
let output = client
    .copy_object()
    .bucket("dest-bucket")
    .key("dest-key")
    .copy_source("source-bucket/source-key")
    .send()
    .await?;
```

#### 追加上传 (AppendObject)

```rust
// 首次追加，position = 0
let output = client
    .append_object()
    .bucket("my-bucket")
    .key("append.txt")
    .position(0)
    .body(b"Hello ".to_vec())
    .send()
    .await?;

// 后续追加
let output = client
    .append_object()
    .bucket("my-bucket")
    .key("append.txt")
    .position(output.next_append_position.unwrap())
    .body(b"World!".to_vec())
    .send()
    .await?;
```

#### 获取对象元信息 (HeadObject)

```rust
let output = client
    .head_object()
    .bucket("my-bucket")
    .key("path/to/file.txt")
    .send()
    .await?;

println!("Content-Length: {:?}", output.content_length);
println!("Content-Type: {:?}", output.content_type);
println!("ETag: {:?}", output.etag);
```

#### 批量删除对象 (DeleteMultipleObjects)

```rust
let output = client
    .delete_multiple_objects()
    .bucket("my-bucket")
    .objects(vec!["file1.txt", "file2.txt", "file3.txt"])
    .quiet(false)
    .send()
    .await?;
```

### 分片上传

#### 初始化分片上传

```rust
let output = client
    .initiate_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .send()
    .await?;

let upload_id = output.upload_id;
```

#### 上传分片

```rust
let output = client
    .upload_part()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(&upload_id)
    .part_number(1)
    .body(part_data)
    .send()
    .await?;
```

#### 完成分片上传

```rust
let output = client
    .complete_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(&upload_id)
    .parts(vec![
        PartInfo { part_number: 1, etag: "etag1".to_string() },
        PartInfo { part_number: 2, etag: "etag2".to_string() },
    ])
    .send()
    .await?;
```

#### 取消分片上传

```rust
let output = client
    .abort_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(&upload_id)
    .send()
    .await?;
```

#### 列出分片

```rust
let output = client
    .list_parts()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(&upload_id)
    .send()
    .await?;
```

#### 列出未完成的分片上传

```rust
let output = client
    .list_multipart_uploads()
    .bucket("my-bucket")
    .send()
    .await?;
```

### Bucket 操作

#### 列出所有 Bucket

```rust
let output = client
    .list_buckets()
    .prefix("my-")
    .max_keys(100)
    .send()
    .await?;

for bucket in output.buckets.bucket {
    println!("Bucket: {}", bucket.name);
}
```

#### 获取 Bucket 信息

```rust
let output = client
    .get_bucket_info()
    .bucket("my-bucket")
    .send()
    .await?;

println!("Bucket: {}", output.bucket.name);
println!("Location: {}", output.bucket.location);
println!("Storage Class: {}", output.bucket.storage_class);
```

#### 获取 Bucket 位置

```rust
let output = client
    .get_bucket_location()
    .bucket("my-bucket")
    .send()
    .await?;

println!("Location: {}", output.location_constraint);
```

#### 获取 Bucket 统计信息

```rust
let output = client
    .get_bucket_stat()
    .bucket("my-bucket")
    .send()
    .await?;

println!("Storage: {} bytes", output.storage.unwrap_or(0));
println!("Object Count: {}", output.object_count.unwrap_or(0));
```

### 其他操作

#### 查询区域信息

```rust
let output = client
    .describe_regions()
    .region("cn-hangzhou")
    .send()
    .await?;

for region in output.region_info {
    println!("Region: {}", region.region);
}
```

#### 恢复归档对象

```rust
let output = client
    .restore_object()
    .bucket("my-bucket")
    .key("archived-file.zip")
    .days(7)
    .tier("Standard")
    .send()
    .await?;
```

#### 表单上传 (PostObject)

```rust
let output = client
    .post_object()
    .bucket("my-bucket")
    .key("uploaded-file.txt")
    .body(file_content)
    .send()
    .await?;
```

## 支持的 API

| API | 描述 |
|-----|------|
| PutObject | 上传对象 |
| GetObject | 下载对象 |
| DeleteObject | 删除对象 |
| CopyObject | 复制对象 |
| AppendObject | 追加上传 |
| HeadObject | 获取对象元信息 |
| GetObjectMeta | 获取对象元数据 |
| DeleteMultipleObjects | 批量删除对象 |
| PostObject | 表单上传 |
| InitiateMultipartUpload | 初始化分片上传 |
| UploadPart | 上传分片 |
| UploadPartCopy | 分片复制 |
| CompleteMultipartUpload | 完成分片上传 |
| AbortMultipartUpload | 取消分片上传 |
| ListParts | 列出分片 |
| ListMultipartUploads | 列出未完成的分片上传 |
| ListObjects | 列出对象 |
| ListBuckets | 列出 Bucket |
| GetBucketInfo | 获取 Bucket 信息 |
| GetBucketLocation | 获取 Bucket 位置 |
| GetBucketStat | 获取 Bucket 统计信息 |
| DescribeRegions | 查询区域信息 |
| RestoreObject | 恢复归档对象 |
| SealAppendObject | 封印追加对象 |

## 功能特性

### TLS 支持

默认使用 `rustls` 作为 TLS 后端，也可以选择系统原生 TLS：

```toml
[dependencies]
oss-sdk-rs = { version = "1.2.4", default-features = false, features = ["default-tls"] }
```

### 错误处理

SDK 提供了详细的错误类型：

```rust
use oss_sdk_rs::OSSError;

match client.get_object().bucket("bucket").key("key").send().await {
    Ok(output) => { /* 处理成功响应 */ },
    Err(OSSError::ServiceError(e)) => {
        println!("服务错误: {} - {}", e.code, e.message);
    },
    Err(OSSError::HttpError(e)) => {
        println!("HTTP 错误: {}", e);
    },
    Err(e) => {
        println!("其他错误: {}", e);
    },
}
```

## 示例

更多示例请查看 [examples](./examples) 目录：

- [`put_object.rs`](./examples/put_object.rs) - 上传对象
- [`get_object.rs`](./examples/get_object.rs) - 下载对象
- [`list_buckets.rs`](./examples/list_buckets.rs) - 列出 Bucket
- [`list_object.rs`](./examples/list_object.rs) - 列出对象
- [`append_object.rs`](./examples/append_object.rs) - 追加上传
- [`get_bucket_info.rs`](./examples/get_bucket_info.rs) - 获取 Bucket 信息
- [`get_bucket_stat.rs`](./examples/get_bucket_stat.rs) - 获取 Bucket 统计
- [`describe_regions.rs`](./examples/describe_regions.rs) - 查询区域信息

## 开发

### 运行测试

```bash
# 单元测试
cargo test

# 集成测试（需要配置环境变量）
cargo test --test integration_test
```

### 运行示例

```bash
# 设置环境变量
export OSS_ACCESS_KEY_ID=your_key_id
export OSS_ACCESS_KEY_SECRET=your_key_secret
export OSS_ENDPOINT=https://oss-cn-hangzhou.aliyuncs.com
export OSS_BUCKET=your-bucket

# 运行示例
cargo run --example put_object
cargo run --example get_object
cargo run --example list_buckets
```

## 参考文档

- [阿里云 OSS API 文档](https://help.aliyun.com/document_detail/31977.html)

## 贡献

欢迎提交 Issue 和 Pull Request！

## License

Apache License 2.0
