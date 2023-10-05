use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::BufRead;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

pub enum HttpVersion {
    One,
    OnePointOne,
}

#[derive(EnumString, Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    Custom { name: String },
}

#[derive(Debug)]
pub struct Headers {
    raw_headers: HashMap<String, Vec<String>>,
}

impl HttpVersion {
    pub fn build(string: &str) -> Result<Self> {
        match string {
            "HTTP/1.0" => Ok(Self::One),
            "HTTP/1.1" => Ok(Self::OnePointOne),
            _ => Err(anyhow!("Unsupported HTTP version")),
        }
    }
}

impl Debug for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::One => f.write_str("HTTP/1.0"),
            HttpVersion::OnePointOne => f.write_str("HTTP/1.1"),
        }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl HttpMethod {
    fn new(method_name: &str) -> Self {
        Self::from_str(method_name).unwrap_or(HttpMethod::Custom {
            name: method_name.to_owned(),
        })
    }
}

impl Headers {
    pub fn new() -> Self {
        Self {
            raw_headers: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.raw_headers
            .entry(key.to_ascii_lowercase())
            .or_insert(Vec::new())
            .push(value.trim().to_owned());
    }

    pub fn set_all(&mut self, key: &str, value: &[&str]) {
        self.raw_headers.insert(
            key.to_owned(),
            value.iter().map(|it| String::from(*it)).collect(),
        );
    }

    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.raw_headers.get(key)
    }

    pub fn get_single(&self, key: &str) -> Option<&String> {
        self.raw_headers.get(key).and_then(|vec| vec.get(0))
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
        self.raw_headers.remove(key)
    }

    pub fn clear(&mut self) {
        self.raw_headers.clear()
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
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
            .map(|body| String::from_utf8_lossy(&body).into_owned())
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
