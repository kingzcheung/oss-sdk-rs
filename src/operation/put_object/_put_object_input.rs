use std::fmt::Display;

#[derive(Debug)]
pub enum ContentDisposition {
    Inline,
    Attachment,
    AttachmentWithFileName(String),
}

impl Display for ContentDisposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentDisposition::Inline => write!(f, "inline"),
            ContentDisposition::Attachment => write!(f, "attachment"),
            ContentDisposition::AttachmentWithFileName(filename) => {
                write!(f, "attachment; filename=\"{}\"", filename)
            }
        }
    }
}
#[derive(Debug)]
pub enum ContentEncoding {
    Identity,
    Gzip,
    Compress,
    Deflate,
    Br,
}

impl Display for ContentEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentEncoding::Identity => write!(f, "identity"),
            ContentEncoding::Gzip => write!(f, "gzip"),
            ContentEncoding::Compress => write!(f, "compress"),
            ContentEncoding::Deflate => write!(f, "deflate"),
            ContentEncoding::Br => write!(f, "br"),
        }
    }
}

#[derive(Debug)]
pub enum ObjectAcl {
    Default,
    Private,
    PublicRead,
    PublicReadWrite,
}

impl Display for ObjectAcl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectAcl::Default => write!(f, "default"),
            ObjectAcl::Private => write!(f, "private"),
            ObjectAcl::PublicRead => write!(f, "public-read"),
            ObjectAcl::PublicReadWrite => write!(f, "public-read-write"),
        }
    }
}

impl Default for ObjectAcl {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug)]
pub enum StorageClass {
    Standard,
    IA,
    Archive,
    ColdArchive,
    DeepColdArchive
}

impl Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageClass::Standard => write!(f, "Standard"),
            StorageClass::IA => write!(f, "IA"),
            StorageClass::Archive => write!(f, "Archive"),
            StorageClass::ColdArchive => write!(f, "ColdArchive"),
            StorageClass::DeepColdArchive => write!(f, "DeepColdArchive"),
        }
    }
}

impl Default for StorageClass {
    fn default() -> Self {
        Self::Standard
    }
}


pub struct PutObjectInput {
    pub(crate) bucket: String,
    pub(crate) key: String,
    pub(crate) body: Vec<u8>,
    pub(crate) content_disposition: Option<ContentDisposition>,
    pub(crate) content_encoding: Option<ContentEncoding>,
    pub(crate) forbid_overwrite: bool,
    pub(crate) object_acl: Option<String>,
    pub(crate) storage_class: Option<String>,
}

#[derive(Debug, Default)]
pub struct PutObjectInputBuilder {
    pub(crate) bucket: Option<String>,
    pub(crate) key: Option<String>,
    pub(crate) body: Option<Vec<u8>>,
    pub(crate) content_disposition: Option<ContentDisposition>,
    pub(crate) content_encoding: Option<ContentEncoding>,
    pub(crate) forbid_overwrite: bool,
    pub(crate) object_acl: Option<ObjectAcl>,
    pub(crate) storage_class: Option<StorageClass>,
}

impl PutObjectInputBuilder {
    pub(crate) fn build(self) -> PutObjectInput {
        PutObjectInput {
            bucket: self.bucket.unwrap_or_default(),
            key: self.key.unwrap_or_default(),
            body: self.body.unwrap_or_default(),
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            forbid_overwrite: self.forbid_overwrite,
            object_acl: self.object_acl.map(|x| x.to_string()),
            storage_class: self.storage_class.map(|x| x.to_string()),
        }
    }

    pub(crate) fn bucket(mut self, input: &str) -> PutObjectInputBuilder {
        self.bucket = Some(input.to_owned());
        self
    }
    pub(crate) fn key(mut self, input: &str) -> PutObjectInputBuilder {
        self.key = Some(input.to_owned());
        self
    }
    pub(crate) fn body(mut self, input: &[u8]) -> PutObjectInputBuilder {
        self.body = Some(input.into());
        self
    }

    pub(crate) fn content_disposition(
        mut self,
        input: ContentDisposition,
    ) -> PutObjectInputBuilder {
        self.content_disposition = Some(input);
        self
    }

    pub(crate) fn content_encoding(mut self, input: ContentEncoding) -> PutObjectInputBuilder {
        self.content_encoding = Some(input);
        self
    }

    pub(crate) fn forbid_overwrite(mut self, input: bool) -> PutObjectInputBuilder {
        self.forbid_overwrite = input;
        self
    }
    pub(crate) fn object_acl(mut self, input: ObjectAcl) -> PutObjectInputBuilder {
        self.object_acl = Some(input);
        self
    }
    pub(crate) fn storage_class(mut self, input: StorageClass) -> PutObjectInputBuilder {
        self.storage_class = Some(input);
        self
    }
}
