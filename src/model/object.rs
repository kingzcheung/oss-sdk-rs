use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Debug,Default)]
pub struct ListBucketResult {
    #[serde(rename(serialize="name", deserialize= "Name"))]
    pub name: String,

    #[serde(rename(serialize="prefix", deserialize= "Prefix"))]
    pub prefix: Option<String>,

    #[serde(rename(serialize="marker", deserialize= "Marker"))]
    pub marker: Option<String>,

    #[serde(rename(serialize="delimiter", deserialize= "Delimiter"))]
    pub delimiter: Option<String>,

    #[serde(rename(serialize="encoding_type", deserialize= "EncodingType"))]
    pub encoding_type: Option<String>,

    
    #[serde(rename(serialize="max_keys", deserialize= "MaxKeys"))]
    pub max_keys: String,
    
    #[serde(rename(serialize="is_truncated", deserialize= "IsTruncated"))]
    pub is_truncated: String,

    #[serde(rename(serialize="next_marker", deserialize= "NextMarker"))]
    pub next_marker: Option<String>,

    #[serde(rename(serialize="contents", deserialize= "Contents"))]
    pub contents: Option<Vec<Content>>,

    #[serde(rename(serialize="common_prefixes", deserialize= "CommonPrefixes"))]
    pub common_prefixes: Option<Vec<CommonPrefixes>>,


    #[serde(rename(serialize="key_count", deserialize= "KeyCount"))]
    pub key_count:Option<usize>
}

#[derive(Serialize, Deserialize,Debug,Default)]
pub struct CommonPrefixes {
    #[serde(rename(serialize="prefix", deserialize= "Prefix"))]
    pub prefix: String,
}

#[derive(Serialize, Deserialize,Debug,Default)]
pub struct Content {
    #[serde(rename(serialize="key", deserialize= "Key"))]
    pub key: String,

    #[serde(rename(serialize="last_modified", deserialize= "LastModified"))]
    pub last_modified: String,

    #[serde(rename(serialize="e_tag", deserialize= "ETag"))]
    pub e_tag: String,

    #[serde(rename(serialize="content_type", deserialize= "Type"))]
    pub content_type: String,

    #[serde(rename(serialize="size", deserialize= "Size"))]
    pub size: String,

    #[serde(rename(serialize="storage_class", deserialize= "StorageClass"))]
    pub storage_class: String,

    #[serde(rename(serialize="owner", deserialize= "Owner"))]
    pub owner: Option<Owner>,
}

#[derive(Serialize, Deserialize,Debug,Default)]
pub struct Owner {
    #[serde(rename(serialize="id", deserialize= "ID"))]
    pub id: String,

    #[serde(rename(serialize="display_name", deserialize= "DisplayName"))]
    pub display_name: String,
}
