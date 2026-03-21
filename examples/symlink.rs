//! Symlink 示例
//! 演示如何创建和获取软链接

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

    let target_key = "examples/original-file.txt";
    let symlink_key = "examples/link-to-file.txt";

    // 先上传一个目标文件
    println!("=== 上传目标文件 ===");
    client
        .put_object()
        .bucket(&bucket)
        .key(target_key)
        .body(b"This is the original file content.".to_vec())
        .content_type("text/plain")
        .send()
        .await?;
    println!("目标文件上传成功: {}", target_key);

    // 创建软链接
    println!("\n=== 创建软链接 ===");
    let output = client
        .put_symlink()
        .bucket(&bucket)
        .key(symlink_key)
        .target(target_key)
        .send()
        .await?;

    println!("软链接创建成功: {}", symlink_key);
    println!("Request ID: {:?}", output.request_id);
    println!("ETag: {:?}", output.etag);

    // 获取软链接信息
    println!("\n=== 获取软链接信息 ===");
    let output = client
        .get_symlink()
        .bucket(&bucket)
        .key(symlink_key)
        .send()
        .await?;

    println!("软链接指向的目标文件: {:?}", output.target);
    println!("ETag: {:?}", output.etag);
    println!("Last Modified: {:?}", output.last_modified);

    // 通过软链接读取文件内容
    println!("\n=== 通过软链接读取文件内容 ===");
    let output = client
        .get_object()
        .bucket(&bucket)
        .key(symlink_key)
        .send()
        .await?;

    let body = output.body.collect().await?;
    let content = String::from_utf8(body.to_vec())?;
    println!("文件内容: {}", content);

    // 创建带 ACL 的软链接
    println!("\n=== 创建带 ACL 的软链接 ===");
    let output = client
        .put_symlink()
        .bucket(&bucket)
        .key("examples/private-link.txt")
        .target(target_key)
        .acl("private")
        .send()
        .await?;

    println!("带 ACL 的软链接创建成功");
    println!("Request ID: {:?}", output.request_id);

    // 创建带存储类型的软链接
    println!("\n=== 创建带存储类型的软链接 ===");
    let output = client
        .put_symlink()
        .bucket(&bucket)
        .key("examples/standard-link.txt")
        .target(target_key)
        .storage_class("Standard")
        .send()
        .await?;

    println!("带存储类型的软链接创建成功");
    println!("Request ID: {:?}", output.request_id);

    // 清理测试文件
    println!("\n=== 清理测试文件 ===");
    client
        .delete_object()
        .bucket(&bucket)
        .key(symlink_key)
        .send()
        .await?;
    println!("软链接已删除: {}", symlink_key);

    client
        .delete_object()
        .bucket(&bucket)
        .key("examples/private-link.txt")
        .send()
        .await?;
    println!("软链接已删除: examples/private-link.txt");

    client
        .delete_object()
        .bucket(&bucket)
        .key("examples/standard-link.txt")
        .send()
        .await?;
    println!("软链接已删除: examples/standard-link.txt");

    client
        .delete_object()
        .bucket(&bucket)
        .key(target_key)
        .send()
        .await?;
    println!("目标文件已删除: {}", target_key);

    println!("\n=== 示例完成 ===");
    Ok(())
}
