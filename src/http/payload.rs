use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;

use super::error::Error;

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

    pub fn as_bytes(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }

    pub fn compress(&self) -> Result<Vec<u8>, Error> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(self.content.as_bytes()).map_err(|_| Error::FailedToCompressPayload)?;
        let compressed_data = encoder.finish().map_err(|_| Error::FailedToCompressPayload)?;
        Ok(compressed_data)
    }
}
