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
    AccessControlListInfo, AppendObjectInput, AppendObjectOutput, BucketInfo, BucketInfoDetail,
    BucketPolicyInfo, CommonPrefix, CopyObjectInput, CopyObjectOutput, DeleteMultipleObjectsInput,
    DeleteMultipleObjectsOutput, DeleteObjectInput, DeleteObjectOutput, DeletedObject,
    DescribeRegionsInput, DescribeRegionsOutput, GetBucketInfoInput, GetBucketInfoOutput,
    GetBucketLocationInput, GetBucketLocationOutput, GetBucketStatInput, GetBucketStatOutput,
    GetObjectInput, GetObjectMetaInput, GetObjectMetaOutput, GetObjectOutput, HeadObjectInput,
    HeadObjectOutput, HeadObjectStatus, ListBucketsInput, ListBucketsOutput, ListObjectsInput,
    ListObjectsOutput, Object, ObjectIdentifier, ObjectType, OwnerInfo, PutObjectInput,
    PutObjectOutput, RegionInfo, RestoreInfo, SealAppendObjectInput, SealAppendObjectOutput,
    ServerSideEncryptionRule,
};

// 重导出 primitives
pub use primitives::{Bucket, ByteStream, Key, Region};
