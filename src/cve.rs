#[derive(PartialEq)]
pub struct Cve {
    pub id: String,
    pub description: String,
    pub published: String,
}

impl Cve {
    pub fn new(id: String, description: String, published: String) -> Self {
        Cve {
            id,
            description,
            published,
        }
    }
}