use std::io::BufRead;

use super::Headers;

#[derive(Debug)]
pub struct HttpBody {
    raw_body: Vec<u8>,
}

impl HttpBody {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            raw_body: Vec::from(bytes),
        }
    }

    pub fn build(headers: &Headers, buf_reader: &mut dyn BufRead) -> Option<Self> {
        let raw = headers
            .get("content-length")
            .and_then(|content_length| content_length[0].parse::<usize>().ok())
            .filter(|content_length| *content_length > 0)
            .and_then(|content_length| {
                let mut body = vec![0u8; content_length];
                match buf_reader.read_exact(&mut body) {
                    Ok(()) => Some(body),
                    Err(_) => None,
                }
            });

        raw.map(|vec| Self::new(&vec))
    }

    pub fn as_str_lossy(&self) -> String {
        String::from_utf8_lossy(&self.raw_body).into_owned()
    }
}

impl Default for HttpBody {
    fn default() -> Self {
        Self {
            raw_body: Default::default(),
        }
    }
}
