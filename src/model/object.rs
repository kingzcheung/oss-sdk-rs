use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Debug,Default)]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

    #[serde(rename = "Marker")]
    pub marker: Option<String>,

    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,

    #[serde(rename = "MaxKeys")]
    pub max_keys: String,

    #[serde(rename = "IsTruncated")]
    pub is_truncated: String,

    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,

    #[serde(rename = "Contents")]
    pub contents: Option<Vec<Content>>,
}

#[derive(Serialize, Deserialize,Debug,Default)]
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

#[derive(Serialize, Deserialize,Debug,Default)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "DisplayName")]
    pub display_name: String,
}
