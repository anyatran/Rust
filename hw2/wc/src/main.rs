use std::io;
use std::os;

fn main() {
    let filename = &os::args()[1];
    let path = Path::new(filename);
    let mut file_final = io::BufferedReader::new(io::File::open(&path));
    let buf: String  = file_final.read_to_string().unwrap();
    let buf_string: &str = buf.as_slice();
    let lines: usize = count_lines(buf_string);
    let words: usize = count_words(buf_string);
    let bytes: usize = count_bytes(buf_string);
    println!("{} {} {} {}", lines, words, bytes, filename);
}

///calculates number of lines
pub fn count_lines(content: &str) -> usize {
    let l: Vec<&str> = content.lines().collect();   
    l.len()
}

///calculates number of words
pub fn count_words(content: &str) -> usize {
    let w: Vec<&str> = content.words().collect();
    w.len()     
}

///calculate the number of bytes
pub fn count_bytes(content: &str) -> usize { 
    content.len()
    
}


#[cfg(test)]

mod wc_test {   
use std::io;
use super::*;
    #[test]
    fn test_wc_1() {
        let p = Path::new("test1.txt");
        let mut f_1 = io::BufferedReader::new(io::File::open(&p));
        let buf_1: String = f_1.read_to_string().unwrap();
        let buf_str_1: &str = buf_1.as_slice();
        assert_eq!(count_lines(buf_str_1), 5);
        assert_eq!(count_words(buf_str_1), 12);
        assert_eq!(count_bytes(buf_str_1), 57);
    }
/// Test for an empty file with 2 spaces
    #[test]
    fn test_wc_2() {
        let p2 = Path::new("test2.txt");
        let mut f_2 = io::BufferedReader::new(io::File::open(&p2));
        let buf_2: String = f_2.read_to_string().unwrap();
        let buf_str_2: &str = buf_2.as_slice();
        assert_eq!(count_lines(buf_str_2), 1);
        assert_eq!(count_words(buf_str_2), 0);
        assert_eq!(count_bytes(buf_str_2), 3);
    }
}
