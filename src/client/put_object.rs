impl super::Client {
    pub fn put_object(&self)-> crate::operation::put_object::PutObjectBuilder {
        crate::operation::put_object::PutObjectBuilder::new(self.handle.clone())
    }
}