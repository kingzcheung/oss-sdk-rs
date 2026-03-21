//! 对象标签操作示例
//!
//! 演示如何设置、获取和删除对象的标签

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取配置
    let access_key_id = env::var("OSS_ACCESS_KEY_ID")?;
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET")?;
    let endpoint = env::var("OSS_ENDPOINT")?;
    let bucket = env::var("OSS_BUCKET")?;

    // 创建凭证和配置
    let credentials = oss_sdk_rs::Credentials::new(access_key_id, access_key_secret);
    let config = oss_sdk_rs::Config::builder()
        .credentials(credentials)
        .endpoint(endpoint)
        .region("cn-hangzhou")
        .build()?;

    // 创建客户端
    let client = oss_sdk_rs::Client::from_config(config)?;

    // 对象名称
    let key = "test-tagging.txt";

    // 1. 先上传一个对象
    println!("=== 上传对象 ===");
    let put_output = client
        .put_object()
        .bucket(&bucket)
        .key(key)
        .body(b"Hello, Tagging!".to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("上传成功，ETag: {:?}", put_output.etag);

    // 2. 设置对象标签
    println!("\n=== 设置对象标签 ===");
    let tag_output = client
        .put_object_tagging()
        .bucket(&bucket)
        .key(key)
        .tag("env", "production")
        .tag("team", "backend")
        .tag("version", "1.0.0")
        .send()
        .await?;

    println!("设置标签成功，Request ID: {}", tag_output.request_id);

    // 3. 获取对象标签
    println!("\n=== 获取对象标签 ===");
    let get_tag_output = client
        .get_object_tagging()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Request ID: {}", get_tag_output.request_id);
    println!("标签列表:");
    for (k, v) in get_tag_output.tags() {
        println!("  {} = {}", k, v);
    }

    // 4. 删除对象标签
    println!("\n=== 删除对象标签 ===");
    let delete_tag_output = client
        .delete_object_tagging()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("删除标签成功，Request ID: {}", delete_tag_output.request_id);

    // 5. 再次获取标签确认已删除
    println!("\n=== 确认标签已删除 ===");
    let get_tag_output2 = client
        .get_object_tagging()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("标签数量: {}", get_tag_output2.tagging.tag_set.len());

    // 6. 清理：删除对象
    println!("\n=== 清理：删除对象 ===");
    let delete_output = client
        .delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("删除对象成功，Request ID: {:?}", delete_output.request_id);

    Ok(())
}
