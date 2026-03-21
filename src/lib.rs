//! OSS SDK for Rust
//! AWS SDK 风格的阿里云 OSS SDK

pub mod errors;
pub mod config;
pub mod credentials;
pub mod primitives;
pub mod types;
pub mod client;
pub mod authv4;
pub mod common;

// 重导出常用类型
pub use config::Config;
pub use credentials::Credentials;
pub use client::Client;
pub use errors::OSSError;

// 重导出 types
pub use types::{
    GetObjectInput, GetObjectOutput,
    PutObjectInput, PutObjectOutput,
    ListObjectsInput, ListObjectsOutput, Object, CommonPrefix,
    ListBucketsInput, ListBucketsOutput, BucketInfo,
    DeleteObjectInput, DeleteObjectOutput,
    HeadObjectInput, HeadObjectOutput,
    CopyObjectInput, CopyObjectOutput,
    DescribeRegionsInput, DescribeRegionsOutput, RegionInfo,
    GetBucketInfoInput, GetBucketInfoOutput, BucketInfoDetail,
    OwnerInfo, AccessControlListInfo, ServerSideEncryptionRule, BucketPolicyInfo,
};

// 重导出 primitives
pub use primitives::{Bucket, Key, Region, ByteStream};