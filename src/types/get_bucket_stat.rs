//! GetBucketStat 操作 Input/Output 类型定义

use serde::{Deserialize, Serialize};

/// GetBucketStat 操作输入
#[derive(Debug, Clone, Default)]
pub struct GetBucketStatInput {
    /// Bucket 名称
    pub bucket: String,
}

impl GetBucketStatInput {
    /// 创建新的 GetBucketStatInput 构建器
    pub fn builder() -> GetBucketStatInputBuilder {
        GetBucketStatInputBuilder::default()
    }
}

/// GetBucketStatInput 构建器
#[derive(Debug, Default)]
pub struct GetBucketStatInputBuilder {
    bucket: Option<String>,
}

impl GetBucketStatInputBuilder {
    /// 设置 Bucket 名称
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    /// 构建 GetBucketStatInput
    pub fn build(self) -> Result<GetBucketStatInput, &'static str> {
        let bucket = self.bucket.ok_or("bucket is required")?;
        Ok(GetBucketStatInput { bucket })
    }
}

/// GetBucketStat 操作输出
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "BucketStat")]
pub struct GetBucketStatOutput {
    /// Bucket 总的实际存储量，单位字节
    #[serde(rename = "Storage", default)]
    pub storage: Option<i64>,

    /// Bucket 中总的 Object 数量
    #[serde(rename = "ObjectCount", default)]
    pub object_count: Option<i64>,

    /// Bucket 中已经初始化但还未完成或者还未中止的 Multipart Upload 数量
    #[serde(rename = "MultipartUploadCount", default)]
    pub multipart_upload_count: Option<i64>,

    /// Bucket 中 Live Channel 的数量
    #[serde(rename = "LiveChannelCount", default)]
    pub live_channel_count: Option<i64>,

    /// Bucket 中上传的 Multipart 分片数量
    #[serde(rename = "MultipartPartCount", default)]
    pub multipart_part_count: Option<i64>,

    /// Bucket 中上传的 Multipart 分片的存储量，单位字节
    #[serde(rename = "MultipartPartStorage", default)]
    pub multipart_part_storage: Option<i64>,

    /// Bucket 中删除标记的数量
    #[serde(rename = "DeleteMarkerCount", default)]
    pub delete_marker_count: Option<i64>,

    /// 获取到的存储信息的时间点，格式为时间戳，单位为秒
    #[serde(rename = "LastModifiedTime", default)]
    pub last_modified_time: Option<i64>,

    // 标准存储类型
    /// 标准存储类型的存储量，单位字节
    #[serde(rename = "StandardStorage", default)]
    pub standard_storage: Option<i64>,

    /// 标准存储类型的 Object 数量
    #[serde(rename = "StandardObjectCount", default)]
    pub standard_object_count: Option<i64>,

    /// Bucket 中上传的标准存储类型的 Multipart 分片数量
    #[serde(rename = "StandardMultipartPartCount", default)]
    pub standard_multipart_part_count: Option<i64>,

    /// Bucket 中上传的标准存储类型的 Multipart 分片的存储量，单位字节
    #[serde(rename = "StandardMultipartPartStorage", default)]
    pub standard_multipart_part_storage: Option<i64>,

    // 低频存储类型
    /// 低频存储类型的计费存储量，单位字节
    #[serde(rename = "InfrequentAccessStorage", default)]
    pub infrequent_access_storage: Option<i64>,

    /// 低频存储类型的实际存储量，单位字节
    #[serde(rename = "InfrequentAccessRealStorage", default)]
    pub infrequent_access_real_storage: Option<i64>,

    /// 低频存储类型的 Object 数量
    #[serde(rename = "InfrequentAccessObjectCount", default)]
    pub infrequent_access_object_count: Option<i64>,

    /// Bucket 中上传的低频存储类型的 Multipart 分片数量
    #[serde(rename = "InfrequentMultipartPartCount", default)]
    pub infrequent_multipart_part_count: Option<i64>,

    /// Bucket 中上传的低频存储类型的 Multipart 分片的存储量，单位字节
    #[serde(rename = "InfrequentMultipartPartStorage", default)]
    pub infrequent_multipart_part_storage: Option<i64>,

    // 归档存储类型
    /// 归档存储类型的计费存储量，单位字节
    #[serde(rename = "ArchiveStorage", default)]
    pub archive_storage: Option<i64>,

    /// 归档存储类型的实际存储量，单位字节
    #[serde(rename = "ArchiveRealStorage", default)]
    pub archive_real_storage: Option<i64>,

    /// 归档存储类型的 Object 数量
    #[serde(rename = "ArchiveObjectCount", default)]
    pub archive_object_count: Option<i64>,

    /// Bucket 中上传的归档存储类型的 Multipart 分片数量
    #[serde(rename = "ArchiveMultipartPartCount", default)]
    pub archive_multipart_part_count: Option<i64>,

    /// Bucket 中上传的归档存储类型的 Multipart 分片的存储量，单位字节
    #[serde(rename = "ArchiveMultipartPartStorage", default)]
    pub archive_multipart_part_storage: Option<i64>,

    // 冷归档存储类型
    /// 冷归档存储类型的计费存储量，单位字节
    #[serde(rename = "ColdArchiveStorage", default)]
    pub cold_archive_storage: Option<i64>,

    /// 冷归档存储类型的实际存储量，单位字节
    #[serde(rename = "ColdArchiveRealStorage", default)]
    pub cold_archive_real_storage: Option<i64>,

    /// 冷归档存储类型的 Object 数量
    #[serde(rename = "ColdArchiveObjectCount", default)]
    pub cold_archive_object_count: Option<i64>,

    /// Bucket 中上传的冷归档存储类型的 Multipart 分片数量
    #[serde(rename = "ColdArchiveMultipartPartCount", default)]
    pub cold_archive_multipart_part_count: Option<i64>,

    /// Bucket 中上传的冷归档存储类型的 Multipart 分片的存储量，单位字节
    #[serde(rename = "ColdArchiveMultipartPartStorage", default)]
    pub cold_archive_multipart_part_storage: Option<i64>,

    // 深度冷归档存储类型
    /// 深度冷归档存储类型的计费存储量，单位字节
    #[serde(rename = "DeepColdArchiveStorage", default)]
    pub deep_cold_archive_storage: Option<i64>,

    /// 深度冷归档存储类型的实际存储量，单位字节
    #[serde(rename = "DeepColdArchiveRealStorage", default)]
    pub deep_cold_archive_real_storage: Option<i64>,

    /// 深度冷归档存储类型的 Object 数量
    #[serde(rename = "DeepColdArchiveObjectCount", default)]
    pub deep_cold_archive_object_count: Option<i64>,

    /// Bucket 中上传的深度冷归档存储类型的 Multipart 分片数量
    #[serde(rename = "DeepColdArchiveMultipartPartCount", default)]
    pub deep_cold_archive_multipart_part_count: Option<i64>,

    /// Bucket 中上传的深度冷归档存储类型的 Multipart 分片的存储量，单位字节
    #[serde(rename = "DeepColdArchiveMultipartPartStorage", default)]
    pub deep_cold_archive_multipart_part_storage: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_get_bucket_stat_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<BucketStat>
  <Storage>1600</Storage>
  <ObjectCount>230</ObjectCount>
  <MultipartUploadCount>40</MultipartUploadCount>
  <LiveChannelCount>4</LiveChannelCount>
  <LastModifiedTime>1643341269</LastModifiedTime>
  <StandardStorage>430</StandardStorage>
  <StandardObjectCount>66</StandardObjectCount>
  <InfrequentAccessStorage>2359296</InfrequentAccessStorage>
  <InfrequentAccessRealStorage>360</InfrequentAccessRealStorage>
  <InfrequentAccessObjectCount>54</InfrequentAccessObjectCount>
  <ArchiveStorage>2949120</ArchiveStorage>
  <ArchiveRealStorage>450</ArchiveRealStorage>
  <ArchiveObjectCount>74</ArchiveObjectCount>
  <ColdArchiveStorage>2359296</ColdArchiveStorage>
  <ColdArchiveRealStorage>360</ColdArchiveRealStorage>
  <ColdArchiveObjectCount>36</ColdArchiveObjectCount>
</BucketStat>"#;

        let output: GetBucketStatOutput = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(output.storage, Some(1600));
        assert_eq!(output.object_count, Some(230));
        assert_eq!(output.multipart_upload_count, Some(40));
        assert_eq!(output.live_channel_count, Some(4));
        assert_eq!(output.last_modified_time, Some(1643341269));

        // 标准存储
        assert_eq!(output.standard_storage, Some(430));
        assert_eq!(output.standard_object_count, Some(66));

        // 低频存储
        assert_eq!(output.infrequent_access_storage, Some(2359296));
        assert_eq!(output.infrequent_access_real_storage, Some(360));
        assert_eq!(output.infrequent_access_object_count, Some(54));

        // 归档存储
        assert_eq!(output.archive_storage, Some(2949120));
        assert_eq!(output.archive_real_storage, Some(450));
        assert_eq!(output.archive_object_count, Some(74));

        // 冷归档存储
        assert_eq!(output.cold_archive_storage, Some(2359296));
        assert_eq!(output.cold_archive_real_storage, Some(360));
        assert_eq!(output.cold_archive_object_count, Some(36));
    }
}
