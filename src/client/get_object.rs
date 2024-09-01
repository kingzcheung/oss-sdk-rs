impl super::Client {
    pub fn get_object(
        &self,
    ) -> crate::operation::get_object::GetObjectBuilder {
        crate::operation::get_object::GetObjectBuilder::new(self.handle.clone())
    }
}
