

#[derive(Debug)]
pub struct HTTPPayload {
    content: String
}

impl HTTPPayload {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn to_string(&self) -> String {
        self.content.clone()
    }

    pub fn content_length(&self) -> usize {
        self.content.len()
    }
}
