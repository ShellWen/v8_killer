pub struct Source {
    pub resource_name: String,
    pub source_string: String,
}

impl Clone for Source {
    fn clone(&self) -> Self {
        Source {
            resource_name: self.resource_name.clone(),
            source_string: self.source_string.clone(),
        }
    }
}
