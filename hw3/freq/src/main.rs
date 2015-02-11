use std::old_io;
use std::old_io::BufferedReader;
use std::collections::HashMap;

fn main() {
    let mut stdin = old_io::BufferedReader::new(old_io::stdin());
    let buf: String = stdin.read_to_string().unwrap();
    let f: HashMap<&str, usize> = freq(&buf);        
    for (key, val) in f.iter() {
        println!("{}: {}", key, val);
    }
}
/// calculates the frequency
pub fn freq<'a>(c: & 'a String) -> HashMap<&'a str, usize> {
    let content: &str = c.as_slice();
    let mut words: HashMap<&str, usize> = HashMap::new();
    for word in content.words() {
        match words.insert(word, 1u) {
            Some(count) =>  { words.insert(word, count + 1) },
            None => {
                words.insert(word, 1u) }
        };
    }
    words
}
