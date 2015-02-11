//! determines the frequencies of unique words in the given file
//! ------------------------------------------------------------------------------------------
//! the problem statement does not specify case-sensitivity, our approach is case-sensitive, so
//! "The" and "the" would be treated as two different words. Additionally, trailing characters
//! are not stripped out and as such, words such as "the", "the!", and "the," are treated as
//! distinct words.

#![allow(unstable)]

use std::io;
use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let mut stdin = io::BufferedReader::new(io::stdin());
    let buf: String = stdin.read_to_string().unwrap();
    let freqs: HashMap<&str, usize> = frequencies(&buf);

    for (word, freq) in freqs.iter() {
        println!("{}: {}", word, freq);
    }
}

fn frequencies<'a>(string: & 'a String) -> HashMap<&'a str, usize> {
    let content: &str = string.as_slice();
    let mut words: HashMap<&str, usize> = HashMap::new();

    for word in content.words() {
        let freq: usize = match words.get(word) {
            Some(num) => *num + 1,
            None      => 1,
        };
        words.insert(word, freq);
    }
    words
}

#[cfg(test)]
mod frequencies_tests {
    use super::frequencies;
    use std::collections::HashMap;

    #[test]
    fn counts_no_words() -> () {
        let e: HashMap<&str, usize> = HashMap::new();
        let s: String = "".to_string();
        assert_eq!(frequencies(&s), e);
    }

    #[test]
    fn counts_single_word() -> () {
        let mut e: HashMap<&str, usize> = HashMap::new();
        e.insert("foo", 1);
        let s: String = "foo".to_string();
        assert_eq!(frequencies(&s), e);
    }

    #[test]
    fn counts_multiple_words() -> () {
        let mut e: HashMap<&str, usize> = HashMap::new();
        e.insert("foo", 2);
        let s: String = "foo foo".to_string();
        assert_eq!(frequencies(&s), e);
    }

    #[test]
    fn counts_different_words() -> () {
        let mut e: HashMap<&str, usize> = HashMap::new();
        e.insert("foo", 2);
        e.insert("bar", 1);
        let s: String = "foo foo bar".to_string();
        assert_eq!(frequencies(&s), e);
    }

    #[test]
    fn distiguishes_between_capitalization() -> () {
        let mut e: HashMap<&str, usize> = HashMap::new();
        e.insert("foo", 1);
        e.insert("Foo", 1);
        e.insert("bar", 1);
        e.insert("bAr", 1);
        let s: String = "foo Foo bar bAr".to_string();
        assert_eq!(frequencies(&s), e);
    }

    #[test]
    fn counts_multiple_lines() -> () {
        let mut e: HashMap<&str, usize> = HashMap::new();
        e.insert("foo", 4);
        let s: String = "foo\nfoo foo\nfoo".to_string();
        assert_eq!(frequencies(&s), e);
    }

    #[test]
    fn distinguishes_words_trailing_characters() -> () {
        let mut e: HashMap<&str, usize> = HashMap::new();
        e.insert("foo", 1);
        e.insert("foo,", 1);
        e.insert("foo!", 1);
        let s: String = "foo foo, foo!".to_string();
        assert_eq!(frequencies(&s), e);
    }

    #[test]
    fn counts_1() {
        let mut expected: HashMap<&str, usize> = HashMap::new();
        expected.insert("nini", 1);
        expected.insert("mimi", 2);
        expected.insert("mimi,", 1);
        expected.insert("hello", 2);
        expected.insert("Anya", 1);
        expected.insert("hEllo", 1);
        expected.insert("anya", 1);
        expected.insert("fdfd", 1);
        let s_slice: &str = "hello anya hello nini fdfd mimi mimi mimi, Anya hEllo";
        let s: String = s_slice.to_string();
        assert_eq!(frequencies(&s), expected);
    }
}
