use std::fmt::Debug;

use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct Headers {
    raw_headers: LinkedHashMap<String, HeaderEntry>,
}

#[derive(Debug)]
struct HeaderEntry {
    original_key: String,
    values: Vec<String>,
}

pub struct Iter<'a> {
    inner_iter: linked_hash_map::Iter<'a, String, HeaderEntry>,
}

impl HeaderEntry {
    fn new(key: &str) -> Self {
        Self {
            original_key: String::from(key),
            values: Vec::new(),
        }
    }
}

impl Headers {
    pub fn new() -> Self {
        Self {
            raw_headers: LinkedHashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.raw_headers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn put(&mut self, key: &str, value: &str) {
        let sanitized_key = sanitize_key(key);

        self.raw_headers
            .entry(sanitized_key)
            .or_insert(HeaderEntry::new(key))
            .values
            .push(value.trim().to_owned());
    }

    pub fn set_all(&mut self, key: &str, values: &[&str]) -> Option<Vec<String>> {
        let old_value = self.remove(key);

        for value in values {
            self.put(key, value);
        }

        old_value
    }

    pub fn get(&self, key: &str) -> Option<&[String]> {
        let key = sanitize_key(key);
        self.raw_headers.get(&key).map(|it| it.values.as_slice())
    }

    pub fn get_first(&self, key: &str) -> Option<&str> {
        self.get(key)
            .map(|values| values[0].as_str())
    }

    pub fn get_splitting_commas(&self, key: &str) -> Option<impl DoubleEndedIterator<Item = &str>> {
        self.get(key).map(|vec| {
            vec.iter()
                .flat_map(|value| value.split(',').map(|s| s.trim()))
        })
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
        let key = sanitize_key(key);
        self.raw_headers.remove(&key).map(|it| it.values)
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
        if let Some((_, value)) = self.inner_iter.next() {
            Some((&value.original_key, &value.values))
        } else {
            None
        }
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
    fn test_initialization() {
        let header = Headers::new();
        assert_eq!(0, header.len());
        assert!(header.is_empty());
    }

    #[test]
    fn test_put_and_get() {
        let mut header = Headers::new();

        header.put("Token", "1234");
        assert_eq!(1, header.len());
        assert_eq!(Some(&["1234".to_owned()][..]), header.get("Token"));

        header.put("TokEn", "Another token");
        assert_eq!(1, header.len());
        assert_eq!(
            Some(&["1234".to_owned(), "Another token".to_owned()][..]),
            header.get("Token")
        );

        assert_eq!(Some("1234"), header.get_first("token"));
    }

    #[test]
    fn test_remove() {
        let mut header = Headers::new();
        header.put("Token", "1234");
        header.put("TokEn", "Another token");

        assert_eq!(
            Some(vec!["1234".to_owned(), "Another token".to_owned()]),
            header.remove("Token")
        );
        assert_eq!(None, header.get("Token"));
    }

    #[test]
    fn test_clear() {
        let mut header = Headers::new();
        header.put("Temp", "Hello");

        header.clear();
        assert!(header.is_empty());
    }

    #[test]
    fn test_set_all() {
        let mut header = Headers::new();

        header.put("maNy", "0");
        assert_eq!(Some(1), header.get("Many").map(|v| v.len()));
        header.set_all("Many", &["1", "2", "3"]);
        assert_eq!(Some(3), header.get("many").map(|v| v.len()));
    }

    #[test]
    fn test_into_iter() {
        let mut header = Headers::new();

        header.put("Content-Legth", "223");
        header.put("Authorization", "Basic sauyfiueyury2387r723vro8w");
        let iter = header.into_iter();
        assert_eq!(2, iter.len());

        assert_eq!(
            223,
            iter.filter(|(key, _)| key == &"Content-Legth")
                .map(|(_, val)| val[0].parse::<i32>().unwrap())
                .take(1)
                .collect::<Vec<_>>()[0]
        );
    }

    #[test]
    fn test_response_string() {
        let mut header = Headers::new();

        header.put("maNy", "0");
        header.set_all("Many", &["1", "2", "3"]);
        header.put("Content-Legth", "223");

        assert_eq!(
            "Many: 1\r\nMany: 2\r\nMany: 3\r\nContent-Legth: 223\r\n",
            header.response_string()
        );
    }

    #[test]
    fn test_get_spliting_comas() {
        let mut headers = Headers::default();

        headers.put("Accept", "application/json");
        headers.put("Accept", "application/html,*/*");

        assert_eq!(1, headers.len());
        assert!(!headers.is_empty());

        let all_accept = headers.get_splitting_commas("Accept").unwrap();

        assert!(["application/json", "application/html", "*/*"]
            .into_iter()
            .eq(all_accept));
    }
}
