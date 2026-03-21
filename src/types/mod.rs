//! 类型定义模块
//! 提供 AWS SDK 风格的 Input/Output 类型

mod abort_multipart_upload;
mod append_object;
mod complete_multipart_upload;
mod copy_object;
mod delete_multiple_objects;
mod delete_object;
mod describe_regions;
mod get_bucket_info;
mod get_bucket_location;
mod get_bucket_stat;
mod get_object;
mod get_object_acl;
mod get_object_meta;
mod get_symlink;
mod head_object;
mod initiate_multipart_upload;
mod list_buckets;
mod list_multipart_uploads;
mod list_objects;
mod list_parts;
mod post_object;
mod put_object;
mod put_object_acl;
mod put_symlink;
mod restore_object;
mod seal_append_object;
mod upload_part;
mod upload_part_copy;

pub use abort_multipart_upload::{
    AbortMultipartUploadError, AbortMultipartUploadInput, AbortMultipartUploadOutput,
};
pub use append_object::{AppendObjectInput, AppendObjectOutput};
pub use complete_multipart_upload::{
    to_complete_multipart_upload_xml, CompleteMultipartUploadInput, CompleteMultipartUploadOutput,
    CompleteMultipartUploadResponse, ObjectAcl, PartItem,
};
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
pub use get_object_acl::{
    AccessControlPolicy, GetObjectAclInput, GetObjectAclOutput, Owner as ObjectOwner,
};
pub use get_object_meta::{GetObjectMetaInput, GetObjectMetaOutput};
pub use get_symlink::{GetSymlinkInput, GetSymlinkOutput};
pub use head_object::{
    HeadObjectInput, HeadObjectOutput, HeadObjectStatus, ObjectType, RestoreInfo,
};
pub use initiate_multipart_upload::{
    InitiateMultipartUploadInput, InitiateMultipartUploadOutput, ServerSideEncryption, StorageClass,
};
pub use list_buckets::{BucketInfo, ListBucketsInput, ListBucketsOutput, Owner};
pub use list_multipart_uploads::{
    CommonPrefix as MultipartCommonPrefix, ListMultipartUploadsInput, ListMultipartUploadsOutput,
    MultipartUpload,
};
pub use list_objects::{CommonPrefix, ListObjectsInput, ListObjectsOutput, Object};
pub use list_parts::{ListPartsInput, ListPartsOutput, PartInfo};
pub use post_object::{PostObjectInput, PostObjectOutput, SuccessActionStatus};
pub use put_object::{
    ContentDisposition, ContentEncoding, ObjectAcl as PutObjectAcl, PutObjectInput,
    PutObjectOutput, StorageClass as PutStorageClass,
};
pub use put_object_acl::{ObjectAcl as ObjectAclPermission, PutObjectAclInput, PutObjectAclOutput};
pub use put_symlink::{PutSymlinkInput, PutSymlinkOutput};
pub use restore_object::{
    to_restore_xml, RestoreObjectInput, RestoreObjectOutput, RestoreStatus, Tier,
};
pub use seal_append_object::{SealAppendObjectInput, SealAppendObjectOutput};
pub use upload_part::{UploadPartInput, UploadPartOutput};
pub use upload_part_copy::{UploadPartCopyInput, UploadPartCopyOutput, UploadPartCopyResponse};
