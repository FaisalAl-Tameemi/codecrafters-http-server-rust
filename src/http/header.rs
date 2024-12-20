
#[derive(Debug)]
pub struct HTTPHeader {
    name: String,
    value: String
}

impl HTTPHeader {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}
