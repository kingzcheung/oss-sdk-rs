//! OSS SDK for Rust
//! AWS SDK 风格的阿里云 OSS SDK

pub mod authv4;
pub mod client;
pub mod common;
pub mod config;
pub mod credentials;
pub mod errors;
pub mod primitives;
pub mod types;

// 重导出常用类型
pub use client::Client;
pub use config::Config;
pub use credentials::Credentials;
pub use errors::OSSError;

// 重导出 types
pub use types::{
    AccessControlListInfo, BucketInfo, BucketInfoDetail, BucketPolicyInfo, CommonPrefix,
    CopyObjectInput, CopyObjectOutput, DeleteObjectInput, DeleteObjectOutput, DescribeRegionsInput,
    DescribeRegionsOutput, GetBucketInfoInput, GetBucketInfoOutput, GetBucketLocationInput,
    GetBucketLocationOutput, GetBucketStatInput, GetBucketStatOutput, GetObjectInput,
    GetObjectOutput, HeadObjectInput, HeadObjectOutput, ListBucketsInput, ListBucketsOutput,
    ListObjectsInput, ListObjectsOutput, Object, OwnerInfo, PutObjectInput, PutObjectOutput,
    RegionInfo, ServerSideEncryptionRule,
};

// 重导出 primitives
pub use primitives::{Bucket, ByteStream, Key, Region};
