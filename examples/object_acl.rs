//! Object ACL 示例
//! 演示如何设置和获取 Object 的访问权限（ACL）

use oss_sdk_rs::{Client, Config, Credentials, ObjectAclPermission};

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

    let key = "examples/acl_demo.txt";

    // 先上传一个测试文件
    println!("=== 上传测试文件 ===");
    client
        .put_object()
        .bucket(&bucket)
        .key(key)
        .body(b"ACL demo content".to_vec())
        .content_type("text/plain")
        .send()
        .await?;
    println!("文件上传成功: {}", key);

    // 获取当前 ACL
    println!("\n=== 获取当前 ACL ===");
    let output = client
        .get_object_acl()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Owner ID: {}", output.owner.id);
    println!("Owner Display Name: {}", output.owner.display_name);
    println!("当前 ACL: {}", output.grant);

    // 设置 ACL 为公共读
    println!("\n=== 设置 ACL 为公共读 ===");
    client
        .put_object_acl()
        .bucket(&bucket)
        .key(key)
        .acl(ObjectAclPermission::PublicRead)
        .send()
        .await?;
    println!("ACL 已设置为 public-read");

    // 再次获取 ACL 验证
    println!("\n=== 验证 ACL 设置 ===");
    let output = client
        .get_object_acl()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("当前 ACL: {}", output.grant);
    assert_eq!(output.grant, ObjectAclPermission::PublicRead);

    // 设置 ACL 为私有
    println!("\n=== 设置 ACL 为私有 ===");
    client
        .put_object_acl()
        .bucket(&bucket)
        .key(key)
        .acl(ObjectAclPermission::Private)
        .send()
        .await?;
    println!("ACL 已设置为 private");

    // 验证私有设置
    let output = client
        .get_object_acl()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("当前 ACL: {}", output.grant);

    // 设置 ACL 为默认（继承 Bucket ACL）
    println!("\n=== 设置 ACL 为默认 ===");
    client
        .put_object_acl()
        .bucket(&bucket)
        .key(key)
        .acl(ObjectAclPermission::Default)
        .send()
        .await?;
    println!("ACL 已设置为 default");

    // 验证默认设置
    let output = client
        .get_object_acl()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("当前 ACL: {}", output.grant);

    // 清理测试文件
    println!("\n=== 清理测试文件 ===");
    client
        .delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;
    println!("测试文件已删除");

    println!("\n=== 示例完成 ===");
    Ok(())
}
