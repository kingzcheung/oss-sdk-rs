//! 类型定义模块
//! 提供 AWS SDK 风格的 Input/Output 类型

mod get_object;
mod put_object;
mod list_objects;
mod list_buckets;
mod delete_object;
mod head_object;
mod copy_object;

pub use get_object::{GetObjectInput, GetObjectOutput};
pub use put_object::{PutObjectInput, PutObjectOutput, ContentDisposition, ContentEncoding, StorageClass, ObjectAcl};
pub use list_objects::{ListObjectsInput, ListObjectsOutput, Object, CommonPrefix};
pub use list_buckets::{ListBucketsInput, ListBucketsOutput, BucketInfo, Owner};
pub use delete_object::{DeleteObjectInput, DeleteObjectOutput};
pub use head_object::{HeadObjectInput, HeadObjectOutput};
pub use copy_object::{CopyObjectInput, CopyObjectOutput};