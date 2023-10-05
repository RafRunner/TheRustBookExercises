use crate::http::{Headers, HttpMethod, HttpVersion};
use anyhow::{anyhow, Result};
use std::io::BufRead;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn build(mut buf_reader: &mut dyn BufRead) -> Result<Self> {
        let request_line = read_utf8_line(&mut buf_reader)?;

        let props = request_line
            .trim()
            .split_ascii_whitespace()
            .collect::<Vec<_>>();

        if props.len() != 3 {
            return Err(anyhow!("Malformed first request line"));
        }

        let mut headers = Headers::new();

        loop {
            let header_line = read_utf8_line(&mut buf_reader)?;
            let header_line = header_line.trim();

            if header_line.is_empty() {
                break;
            }

            if let Some((key, value)) = header_line.split_once(':') {
                headers.set(key, value);
            }
        }

        let body = read_body(&headers, &mut buf_reader);

        let method = HttpMethod::new(props[0]);
        let path = props[1].to_owned();
        let version = HttpVersion::build(props[2])?;

        Ok(Self {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    pub fn body_as_string(&self) -> Option<String> {
        self.body
            .as_ref()
            .map(|body| String::from_utf8_lossy(body).into_owned())
    }
}

fn read_utf8_line(buf_reader: &mut dyn BufRead) -> Result<String> {
    let mut request_line = Vec::new();
    buf_reader.read_until(b'\n', &mut request_line)?;

    String::from_utf8(request_line).map_err(|_| anyhow!("Unexpected non UTF-8 string"))
}

fn read_body(headers: &Headers, buf_reader: &mut dyn BufRead) -> Option<Vec<u8>> {
    headers
        .get("content-length")
        .and_then(|content_length| content_length[0].parse::<usize>().ok())
        .filter(|content_length| *content_length > 0)
        .and_then(|content_length| {
            let mut body = vec![0u8; content_length];
            match buf_reader.read_exact(&mut body) {
                Ok(()) => Some(body),
                Err(_) => None,
            }
        })
}
