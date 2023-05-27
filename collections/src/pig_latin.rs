// Convert strings to pig latin. The first consonant of each word is moved to the end
// of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
// a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in
// mind the details about UTF-8 encoding!

use regex::{Regex, Captures};

pub fn pigfy(frase: &str) -> String {
    if frase.is_empty() {
        return String::new();
    }

    frase
        .split_whitespace()
        .map(self::pigfy_word)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn pigfy_word(word: &str) -> String {
    let re = Regex::new(r"^([^a-zA-Z]*)([a-zA-Z]+)(.*)").unwrap();
    let caps = re.captures(word).unwrap();

    let before = caps.get_or_empty(1);
    let alpha = caps.get(2).map_or(String::new(), |m| {
        let word = m.as_str().to_owned();
        word
        .chars()
        .next()
        .map(|c| {
            if c.is_vowel() {
                format!("{}-hay", word)
            } else {
                format!("{}-{}ay", &word[1..], c)
            }
        })
        .unwrap_or(String::new())
    });
    let after = caps.get_or_empty(3);

    format!("{}{}{}", before, alpha, after)
}

trait CharExtensions {
    fn is_vowel(&self) -> bool;
}

impl CharExtensions for char {
    fn is_vowel(&self) -> bool {
        match self {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}

trait CapturesExtensions<'a> {
    fn get_or_empty(&self, index: usize) -> &'a str;
}

impl<'a> CapturesExtensions<'a> for Captures<'a> {
    fn get_or_empty(&self, index: usize) -> &'a str {
        self.get(index).map_or("", |m| m.as_str())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pigfy_words() {
        assert_eq!("apple-hay", pigfy_word("apple"));
        assert_eq!("irst-fay", pigfy_word("first"));
        assert_eq!("end-hay", pigfy_word("end"));
        assert_eq!("ecord-Ray", pigfy_word("Record"))
    }

    #[test]
    fn pigfy_frases() {
        assert_eq!("o-Tay e-bay or-hay ot-nay o-tay e-bay, hat-tay is-hay he-tay uestion-qay", pigfy("To be or not to be, that is the question"))
    }
}
