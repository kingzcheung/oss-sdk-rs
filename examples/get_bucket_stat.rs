//! GetBucketStat 示例
//! 演示如何获取 Bucket 的存储统计信息

use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载配置
    let access_key_id = std::env::var("OSS_ACCESS_KEY_ID")?;
    let access_key_secret = std::env::var("OSS_ACCESS_KEY_SECRET")?;
    let endpoint = std::env::var("OSS_ENDPOINT")
        .unwrap_or_else(|_| "https://oss-cn-hangzhou.aliyuncs.com".to_string());
    let bucket = std::env::var("OSS_BUCKET").unwrap_or_else(|_| "my-bucket".to_string());

    // 创建凭证
    let credentials = Credentials::new(access_key_id, access_key_secret);

    // 创建配置
    let config = Config::builder()
        .credentials(credentials)
        .endpoint(endpoint)
        .region("cn-hangzhou")
        .build()?;

    // 创建客户端
    let client = Client::from_config(config)?;

    // 获取 Bucket 统计信息
    let output = client.get_bucket_stat().bucket(&bucket).send().await?;

    println!("=== Bucket 统计信息 ===");
    println!("Bucket: {}", bucket);

    if let Some(storage) = output.storage {
        println!(
            "总存储量: {} 字节 ({:.2} MB)",
            storage,
            storage as f64 / 1024.0 / 1024.0
        );
    }

    if let Some(count) = output.object_count {
        println!("Object 数量: {}", count);
    }

    if let Some(count) = output.multipart_upload_count {
        println!("未完成的 Multipart Upload 数量: {}", count);
    }

    if let Some(count) = output.live_channel_count {
        println!("Live Channel 数量: {}", count);
    }

    if let Some(count) = output.multipart_part_count {
        println!("Multipart 分片数量: {}", count);
    }

    if let Some(storage) = output.multipart_part_storage {
        println!("Multipart 分片存储量: {} 字节", storage);
    }

    if let Some(count) = output.delete_marker_count {
        println!("删除标记数量: {}", count);
    }

    if let Some(time) = output.last_modified_time {
        println!("统计信息时间戳: {}", time);
    }

    println!("\n=== 标准存储 ===");
    if let Some(storage) = output.standard_storage {
        println!("存储量: {} 字节", storage);
    }
    if let Some(count) = output.standard_object_count {
        println!("Object 数量: {}", count);
    }

    println!("\n=== 低频存储 ===");
    if let Some(storage) = output.infrequent_access_storage {
        println!("计费存储量: {} 字节", storage);
    }
    if let Some(storage) = output.infrequent_access_real_storage {
        println!("实际存储量: {} 字节", storage);
    }
    if let Some(count) = output.infrequent_access_object_count {
        println!("Object 数量: {}", count);
    }

    println!("\n=== 归档存储 ===");
    if let Some(storage) = output.archive_storage {
        println!("计费存储量: {} 字节", storage);
    }
    if let Some(storage) = output.archive_real_storage {
        println!("实际存储量: {} 字节", storage);
    }
    if let Some(count) = output.archive_object_count {
        println!("Object 数量: {}", count);
    }

    println!("\n=== 冷归档存储 ===");
    if let Some(storage) = output.cold_archive_storage {
        println!("计费存储量: {} 字节", storage);
    }
    if let Some(storage) = output.cold_archive_real_storage {
        println!("实际存储量: {} 字节", storage);
    }
    if let Some(count) = output.cold_archive_object_count {
        println!("Object 数量: {}", count);
    }

    println!("\n=== 深度冷归档存储 ===");
    if let Some(storage) = output.deep_cold_archive_storage {
        println!("计费存储量: {} 字节", storage);
    }
    if let Some(storage) = output.deep_cold_archive_real_storage {
        println!("实际存储量: {} 字节", storage);
    }
    if let Some(count) = output.deep_cold_archive_object_count {
        println!("Object 数量: {}", count);
    }

    Ok(())
}
