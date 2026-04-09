#[derive(Debug, Clone)]
pub(crate) struct Genre {
    id: i32,
    name: String,
}

impl Genre {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
