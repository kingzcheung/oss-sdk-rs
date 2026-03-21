//! AppendObject 示例
//! 演示如何以追加写的方式上传文件

use oss_sdk_rs::{Client, Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载配置
    let access_key_id = std::env::var("OSS_ACCESS_KEY_ID")?;
    let access_key_secret = std::env::var("OSS_ACCESS_KEY_SECRET")?;
    let endpoint = std::env::var("OSS_ENDPOINT").unwrap_or_else(|_| "https://oss-cn-hangzhou.aliyuncs.com".to_string());
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

    let key = "examples/append_demo.txt";

    // 清理可能存在的旧对象
    let _ = client.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await;

    println!("=== AppendObject 示例 ===\n");

    // 首次追加（position = 0）
    println!("1. 首次追加 (position=0)");
    let output = client.append_object()
        .bucket(&bucket)
        .key(key)
        .position(0)
        .body(b"Hello ".to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("   ETag: {:?}", output.etag);
    println!("   Next position: {:?}", output.next_append_position);
    println!("   CRC64: {:?}", output.hash_crc64ecma);

    let mut next_pos = output.next_append_position.unwrap();

    // 第二次追加
    println!("\n2. 第二次追加 (position={})", next_pos);
    let output = client.append_object()
        .bucket(&bucket)
        .key(key)
        .position(next_pos)
        .body(b"OSS ".to_vec())
        .send()
        .await?;

    println!("   Next position: {:?}", output.next_append_position);
    next_pos = output.next_append_position.unwrap();

    // 第三次追加
    println!("\n3. 第三次追加 (position={})", next_pos);
    let output = client.append_object()
        .bucket(&bucket)
        .key(key)
        .position(next_pos)
        .body(b"SDK!".to_vec())
        .send()
        .await?;

    println!("   Next position: {:?}", output.next_append_position);

    // 读取最终内容
    println!("\n4. 读取最终内容");
    let get_output = client.get_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    let body = get_output.body.collect().await?;
    let content = String::from_utf8(body.to_vec())?;
    println!("   内容: {}", content);

    // 清理
    println!("\n5. 清理测试文件");
    client.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("   已删除");

    Ok(())
}
