
#[derive(Debug)]
pub struct HTTPHeader {
    name: String,
    value: String
}

impl HTTPHeader {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
