
// Convert strings to pig latin. The first consonant of each word is moved to the end
// of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
// a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in
// mind the details about UTF-8 encoding!

pub fn pigfy(frase: &str) -> String {
    if frase.is_empty() {
        return String::new();
    }

    let mut pig = String::new();

    for word in frase.split_whitespace() {
        pig.push_str(&pigfy_word(word));
        pig.push(' ');
    }

    return pig;
}

pub fn pigfy_word(word: &str) -> String {
    let mut iter = word.chars();
    if let Some(first_letter) = iter.next() {
        return match first_letter.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
            _ => {
                let minus_first = iter.as_str();
                format!("{}-{}ay", minus_first, first_letter)
            }
        };
    }

    return String::new();
}
