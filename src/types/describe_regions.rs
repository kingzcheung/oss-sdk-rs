//! DescribeRegions 操作 Input/Output 类型定义

use serde::{Deserialize, Serialize};

/// DescribeRegions 操作输入
#[derive(Debug, Clone, Default)]
pub struct DescribeRegionsInput {
    /// 指定 OSS 专用 Region ID
    /// 如果不指定，则返回所有支持地域对应的 Endpoint 信息
    pub region: Option<String>,
}

impl DescribeRegionsInput {
    /// 创建新的 DescribeRegionsInput 构建器
    pub fn builder() -> DescribeRegionsInputBuilder {
        DescribeRegionsInputBuilder::default()
    }
}

/// DescribeRegionsInput 构建器
#[derive(Debug, Default)]
pub struct DescribeRegionsInputBuilder {
    region: Option<String>,
}

impl DescribeRegionsInputBuilder {
    /// 设置 OSS 专用 Region ID
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// 构建 DescribeRegionsInput
    pub fn build(self) -> DescribeRegionsInput {
        DescribeRegionsInput {
            region: self.region,
        }
    }
}

/// DescribeRegions 操作输出
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "RegionInfoList")]
pub struct DescribeRegionsOutput {
    /// 地域信息列表
    #[serde(rename = "RegionInfo", default)]
    pub region_info_list: Vec<RegionInfo>,
}

/// 地域信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionInfo {
    /// OSS 专用 Region ID
    #[serde(rename = "Region")]
    pub region: String,
    /// 外网 Endpoint
    #[serde(rename = "InternetEndpoint")]
    pub internet_endpoint: String,
    /// 内网 Endpoint
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: String,
    /// 传输加速 Endpoint
    #[serde(rename = "AccelerateEndpoint")]
    pub accelerate_endpoint: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_describe_regions_output() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<RegionInfoList>
  <RegionInfo>
     <Region>oss-cn-hangzhou</Region>
     <InternetEndpoint>oss-cn-hangzhou.aliyuncs.com</InternetEndpoint>
     <InternalEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</InternalEndpoint>
     <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>  
  </RegionInfo>
  <RegionInfo>
     <Region>oss-cn-shanghai</Region>
     <InternetEndpoint>oss-cn-shanghai.aliyuncs.com</InternetEndpoint>
     <InternalEndpoint>oss-cn-shanghai-internal.aliyuncs.com</InternalEndpoint>
     <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>  
  </RegionInfo>
</RegionInfoList>"#;

        let output: DescribeRegionsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.region_info_list.len(), 2);
        
        let first = &output.region_info_list[0];
        assert_eq!(first.region, "oss-cn-hangzhou");
        assert_eq!(first.internet_endpoint, "oss-cn-hangzhou.aliyuncs.com");
        assert_eq!(first.internal_endpoint, "oss-cn-hangzhou-internal.aliyuncs.com");
        assert_eq!(first.accelerate_endpoint, "oss-accelerate.aliyuncs.com");
        
        let second = &output.region_info_list[1];
        assert_eq!(second.region, "oss-cn-shanghai");
        assert_eq!(second.internet_endpoint, "oss-cn-shanghai.aliyuncs.com");
    }

    #[test]
    fn test_deserialize_single_region() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<RegionInfoList>
  <RegionInfo>
    <Region>oss-cn-hangzhou</Region>
    <InternetEndpoint>oss-cn-hangzhou.aliyuncs.com</InternetEndpoint>
    <InternalEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</InternalEndpoint>
    <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>  
  </RegionInfo>
</RegionInfoList>"#;

        let output: DescribeRegionsOutput = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(output.region_info_list.len(), 1);
        
        let first = &output.region_info_list[0];
        assert_eq!(first.region, "oss-cn-hangzhou");
        assert_eq!(first.internet_endpoint, "oss-cn-hangzhou.aliyuncs.com");
        assert_eq!(first.internal_endpoint, "oss-cn-hangzhou-internal.aliyuncs.com");
        assert_eq!(first.accelerate_endpoint, "oss-accelerate.aliyuncs.com");
    }
}