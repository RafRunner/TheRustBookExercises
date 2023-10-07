use crate::http::{Headers, HttpVersion};
use std::io::Write;
use std::net::TcpStream;

use super::HttpStatus;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpStatus,
    pub headers: Headers,
    pub entity: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new(status: HttpStatus) -> Self {
        Self {
            headers: Headers::new(),
            version: HttpVersion::OnePointOne,
            entity: None,
            status,
        }
    }

    pub fn entity(&mut self, entity: &[u8], content_type: &str) {
        self.headers
            .put("Content-Length", &entity.len().to_string());
        self.headers.put("Content-Type", content_type);

        self.entity = Some(Vec::from(entity));
    }

    pub fn str_entity(&mut self, entity: &str, content_type: &str) {
        self.entity(entity.as_bytes(), content_type);
    }

    pub fn write(&self, mut stream: TcpStream) {
        let response = format!(
            "{} {} {}\r\n{}\r\n",
            self.version,
            self.status.code(),
            self.status.reason_phrase(),
            self.headers.response_string()
        );
        let mut bytes = Vec::from(response.as_bytes());
        if let Some(entity) = &self.entity {
            for byte in entity {
                bytes.push(*byte);
            }
        }

        stream.write_all(&bytes).unwrap();
    }
}
