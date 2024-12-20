
#[derive(Debug, Clone)]
pub struct HTTPHeader {
    pub name: String,
    pub value: String
}

impl HTTPHeader {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}

impl From<&str> for HTTPHeader {
    fn from(value: &str) -> Self {
        let parts = value.split(": ").collect::<Vec<&str>>();
        Self::new(parts[0].to_string(), parts[1].to_string())
    }
}