use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Debug)]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "MaxKeys")]
    pub max_keys: String,

    #[serde(rename = "IsTruncated")]
    pub is_truncated: String,

    #[serde(rename = "Contents")]
    pub contents: Vec<Content>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Content {
    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "LastModified")]
    pub last_modified: String,

    #[serde(rename = "ETag")]
    pub e_tag: String,

    #[serde(rename = "Type")]
    pub content_type: String,

    #[serde(rename = "Size")]
    pub size: String,

    #[serde(rename = "StorageClass")]
    pub storage_class: String,

    #[serde(rename = "Owner")]
    pub owner: Owner,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "DisplayName")]
    pub display_name: String,
}
