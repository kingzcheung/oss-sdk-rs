
pub struct GetObjectInput {
    pub(crate) bucket: Option<String>,
    pub(crate) key: Option<String>,
}

#[derive(Default,Debug,Clone)]
pub struct GetObjectInputBuilder {
    pub(crate) bucket: Option<String>,
    pub(crate) key: Option<String>,
}

impl GetObjectInputBuilder {
    pub fn bucket<S:AsRef<str>>(mut self, input: S)->Self {
        self.bucket = Some(input.as_ref().into());
        self
    }
    pub fn key<S:AsRef<str>>(mut self, input: S)->Self {
        self.key = Some(input.as_ref().into());
        self
    }
    
    pub(crate) fn build(self) -> GetObjectInput {
        GetObjectInput { bucket: self.bucket, key: self.key }
    }

}