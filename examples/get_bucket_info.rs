//! GetBucketInfo 示例
//! 演示如何获取 Bucket 的详细信息

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

    // 获取 Bucket 信息
    let output = client.get_bucket_info().bucket(&bucket).send().await?;

    let info = &output.bucket;

    println!("=== Bucket 信息 ===");
    println!("名称: {}", info.name);
    println!("地域: {}", info.location);
    println!("存储类型: {}", info.storage_class);
    println!("创建时间: {}", info.creation_date);
    println!("外网 Endpoint: {}", info.extranet_endpoint);
    println!("内网 Endpoint: {}", info.intranet_endpoint);

    println!("\n=== 拥有者信息 ===");
    println!("ID: {}", info.owner.id);
    println!("显示名称: {}", info.owner.display_name);

    println!("\n=== 访问控制 ===");
    println!("ACL: {}", info.access_control_list.grant);

    if let Some(access_monitor) = &info.access_monitor {
        println!("访问跟踪状态: {}", access_monitor);
    }

    if let Some(transfer_acceleration) = &info.transfer_acceleration {
        println!("传输加速状态: {}", transfer_acceleration);
    }

    if let Some(cross_region_replication) = &info.cross_region_replication {
        println!("跨区域复制状态: {}", cross_region_replication);
    }

    if let Some(versioning) = &info.versioning {
        println!("版本控制状态: {}", versioning);
    }

    if let Some(data_redundancy_type) = &info.data_redundancy_type {
        println!("数据容灾类型: {}", data_redundancy_type);
    }

    if let Some(resource_group_id) = &info.resource_group_id {
        println!("资源组 ID: {}", resource_group_id);
    }

    if let Some(comment) = &info.comment {
        println!("备注: {}", comment);
    }

    if let Some(block_public_access) = &info.block_public_access {
        println!("阻止公共访问: {}", block_public_access);
    }

    if let Some(encryption) = &info.server_side_encryption_rule {
        println!("\n=== 服务端加密 ===");
        if let Some(algorithm) = &encryption.sse_algorithm {
            println!("加密算法: {}", algorithm);
        }
        if let Some(key_id) = &encryption.kms_master_key_id {
            println!("KMS 密钥 ID: {}", key_id);
        }
        if let Some(data_encryption) = &encryption.kms_data_encryption {
            println!("数据加密算法: {}", data_encryption);
        }
    }

    if let Some(policy) = &info.bucket_policy {
        println!("\n=== 日志策略 ===");
        if let Some(log_bucket) = &policy.log_bucket {
            println!("日志 Bucket: {}", log_bucket);
        }
        if let Some(log_prefix) = &policy.log_prefix {
            println!("日志前缀: {}", log_prefix);
        }
    }

    Ok(())
}
