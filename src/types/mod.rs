//! 类型定义模块
//! 提供 AWS SDK 风格的 Input/Output 类型

mod append_object;
mod copy_object;
mod delete_multiple_objects;
mod delete_object;
mod describe_regions;
mod get_bucket_info;
mod get_bucket_location;
mod get_bucket_stat;
mod get_object;
mod get_object_meta;
mod head_object;
mod list_buckets;
mod list_objects;
mod post_object;
mod put_object;
mod restore_object;
mod seal_append_object;

pub use append_object::{AppendObjectInput, AppendObjectOutput};
pub use copy_object::{CopyObjectInput, CopyObjectOutput};
pub use delete_multiple_objects::{
    to_xml, DeleteMultipleObjectsInput, DeleteMultipleObjectsOutput, DeletedObject,
    ObjectIdentifier,
};
pub use delete_object::{DeleteObjectInput, DeleteObjectOutput};
pub use describe_regions::{DescribeRegionsInput, DescribeRegionsOutput, RegionInfo};
pub use get_bucket_info::{
    AccessControlListInfo, BucketInfoDetail, BucketPolicyInfo, GetBucketInfoInput,
    GetBucketInfoOutput, OwnerInfo, ServerSideEncryptionRule,
};
pub use get_bucket_location::{GetBucketLocationInput, GetBucketLocationOutput};
pub use get_bucket_stat::{GetBucketStatInput, GetBucketStatOutput};
pub use get_object::{GetObjectInput, GetObjectOutput};
pub use get_object_meta::{GetObjectMetaInput, GetObjectMetaOutput};
pub use head_object::{
    HeadObjectInput, HeadObjectOutput, HeadObjectStatus, ObjectType, RestoreInfo,
};
pub use list_buckets::{BucketInfo, ListBucketsInput, ListBucketsOutput, Owner};
pub use list_objects::{CommonPrefix, ListObjectsInput, ListObjectsOutput, Object};
pub use post_object::{PostObjectInput, PostObjectOutput, SuccessActionStatus};
pub use put_object::{
    ContentDisposition, ContentEncoding, ObjectAcl, PutObjectInput, PutObjectOutput, StorageClass,
};
pub use restore_object::{to_restore_xml, RestoreObjectInput, RestoreObjectOutput, RestoreStatus, Tier};
pub use seal_append_object::{SealAppendObjectInput, SealAppendObjectOutput};
