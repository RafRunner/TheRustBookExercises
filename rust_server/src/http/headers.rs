use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Headers {
    raw_headers: HashMap<String, Vec<String>>,
}

pub struct Iter<'a> {
    inner_iter: std::collections::hash_map::Iter<'a, String, Vec<String>>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            raw_headers: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.raw_headers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn put(&mut self, key: &str, value: &str) {
        let key = sanitize_key(key);

        self.raw_headers
            .entry(key)
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
        let key = sanitize_key(key);
        self.raw_headers.get(&key)
    }

    pub fn get_first(&self, key: &str) -> Option<&String> {
        self.get(key).and_then(|vec| vec.get(0))
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
        let key = sanitize_key(key);
        self.raw_headers.remove(&key)
    }

    pub fn clear(&mut self) {
        self.raw_headers.clear()
    }

    pub fn response_string(&self) -> String {
        let mut buffer = String::new();

        for (key, values) in self {
            for value in values {
                buffer.push_str(&format!("{key}: {value}\r\n"));
            }
        }

        buffer
    }
}

fn sanitize_key(key: &str) -> String {
    key.to_lowercase()
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a String, &'a Vec<String>);

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner_iter: self.raw_headers.iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a String, &'a Vec<String>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner_iter.len(), Some(self.inner_iter.len()))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_basics() {
        let mut header = Headers::new();
        assert_eq!(0, header.len());

        header.put("Token", "1234");
        assert_eq!(1, header.len());
        assert_eq!(Some(&vec!["1234".to_owned()]), header.get("Token"));

        header.put("TokEn", "Another token");
        assert_eq!(1, header.len());
        assert_eq!(
            Some(&vec!["1234".to_owned(), "Another token".to_owned()]),
            header.get("Token")
        );
        assert_eq!(Some(&"1234".to_owned()), header.get_first("token"));

        assert_eq!(
            Some(vec!["1234".to_owned(), "Another token".to_owned()]),
            header.remove("Token")
        );
        assert_eq!(None, header.get("Token"));

        header.put("Temp", "Hello");

        header.clear();
        assert!(header.is_empty());

        header.put("maNy", "0");
        assert_eq!(Some(1), header.get("Many").map(|v| v.len()));
        header.set_all("many", &["1", "2", "3"]);
        assert_eq!(Some(3), header.get("Many").map(|v| v.len()));

        header.put("Content-Legth", "223");
        let iter = header.into_iter();
        assert_eq!(2, iter.len());

        assert_eq!(
            223,
            iter.filter(|(key, _)| key == &"content-legth")
                .map(|(_, val)| val[0].parse::<i32>().unwrap())
                .take(1)
                .collect::<Vec<_>>()[0]
        );

        assert_eq!(
            "many: 1\r\nmany: 2\r\nmany: 3\r\ncontent-legth: 223\r\n",
            header.response_string()
        );
    }
}
