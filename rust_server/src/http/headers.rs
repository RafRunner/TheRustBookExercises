use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct Headers {
    raw_headers: HashMap<String, Vec<String>>,
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

impl Display for Headers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, values) in self.raw_headers.iter() {
            f.write_fmt(format_args!("{key}: "))?;
            for value in values {
                f.write_fmt(format_args!("{value}"))?;
            }
            f.write_str("\r\n")?;
        }
        f.write_str("")
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}
