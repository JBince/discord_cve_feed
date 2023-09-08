#[derive(PartialEq)]
pub struct Cve {
    pub id: String,
    pub description: String,
}

impl Cve {
    pub fn new(id: String, description: String) -> Self {
        Cve {
            id,
            description,
        }
    }
}