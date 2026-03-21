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
    AbortMultipartUploadError, AbortMultipartUploadInput, AbortMultipartUploadOutput,
    AccessControlListInfo, AppendObjectInput, AppendObjectOutput, BucketInfo, BucketInfoDetail,
    BucketPolicyInfo, CommonPrefix, CompleteMultipartUploadInput, CompleteMultipartUploadOutput,
    CompleteMultipartUploadResponse, CopyObjectInput, CopyObjectOutput, DeleteMultipleObjectsInput,
    DeleteMultipleObjectsOutput, DeleteObjectInput, DeleteObjectOutput, DeletedObject,
    DescribeRegionsInput, DescribeRegionsOutput, GetBucketInfoInput, GetBucketInfoOutput,
    GetBucketLocationInput, GetBucketLocationOutput, GetBucketStatInput, GetBucketStatOutput,
    GetObjectAclInput, GetObjectAclOutput, GetObjectInput, GetObjectMetaInput, GetObjectMetaOutput,
    GetObjectOutput, GetSymlinkInput, GetSymlinkOutput, HeadObjectInput, HeadObjectOutput,
    HeadObjectStatus, InitiateMultipartUploadInput, InitiateMultipartUploadOutput,
    ListBucketsInput, ListBucketsOutput, ListMultipartUploadsInput, ListMultipartUploadsOutput,
    ListObjectsInput, ListObjectsOutput, ListPartsInput, ListPartsOutput, MultipartUpload, Object,
    ObjectAcl, ObjectAclPermission, ObjectIdentifier, ObjectType, OwnerInfo, PartInfo, PartItem,
    PostObjectInput, PostObjectOutput, PutObjectAclInput, PutObjectAclOutput, PutObjectInput,
    PutObjectOutput, PutSymlinkInput, PutSymlinkOutput, RegionInfo, RestoreInfo,
    RestoreObjectInput, RestoreObjectOutput, RestoreStatus, SealAppendObjectInput,
    SealAppendObjectOutput, ServerSideEncryption, ServerSideEncryptionRule, StorageClass,
    SuccessActionStatus, Tier, UploadPartCopyInput, UploadPartCopyOutput, UploadPartCopyResponse,
    UploadPartInput, UploadPartOutput,
};

// 重导出 primitives
pub use primitives::{Bucket, ByteStream, Key, Region};
