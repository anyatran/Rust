use std::io;
use std::io::BufferedReader;

fn main() {

    let end_marker: &str = "999";
    let stdin = io::BufferedReader::new(io::stdin());
    let measurements: Vec<f64> = load_measurements(stdin, end_marker);

    if measurements.len() == 0 {
        println!("0\n0\n0");
    } else {
        let average: f64 = avg_float_vec(&measurements);
        let plus_five: usize = num_within_range(&measurements, average, average+5.0);
        let minus_five: usize = num_within_range(&measurements, average-5.0, average);
        println!("{}\n{}\n{}", average, plus_five, minus_five);
    }
}

/// builds up a vector of floats parsed from each line in the provided reader or until
/// the given "end_icator" is seen
fn load_measurements<R: Reader>(mut buff: io::BufferedReader<R>, end_icator: &str) -> Vec<f64> {
    let mut measurements: Vec<f64> = vec![];

    for line in buff.lines() {
        match parse_line(line, end_icator) {
            Ok(num) => measurements.push(num),
            Err("eof") => break,
            Err(_) => continue,

        }
    }
    measurements
}

/// parses the given line into a valid measurement
pub fn parse_line(line: io::IoResult<String>, end_icator: &str) -> Result<f64, &str> {

    let input_line = match line.ok() {
        Some(input_line) => input_line,
        None => return Err("unable to parse input line"),
    };

    // trim away extra whitespace
    let trimmed = input_line.trim();

    if trimmed == end_icator {
        return Err("eof")
    }

    // try to parse trimmed line into 64-bit float
    let input_num: Option<f64> = trimmed.parse();

    let measurement = match input_num {
        Some(measurement) => measurement,
        None => return Err("unable to parse string into float"),
    };

    if measurement < 0.0 {
        Err("invalid measurement, value below 0")
    } else {
        Ok(measurement)
    }
}

/// returns the mean of the given vector of floating points
pub fn avg_float_vec(lof: &Vec<f64>) -> f64 {
    let num_samples: f64 = lof.len() as f64; // len() -> usize, must "cast"(?)
    let sum_total: f64 = lof.iter().fold(0.0, |acc, &m| acc + m);
    sum_total / num_samples
}

/// returns the number of floats in the given Vector of floats that are within
/// the given boundaries (inclusive)
pub fn num_within_range(lof: &Vec<f64>, lower_bound: f64, upper_bound: f64) -> usize {
    // |&: ... | { ... } creates closure/lambda
    let in_range = |&: n: f64| -> bool {
        (lower_bound <= n) && (n <= upper_bound)
    };
    lof.iter().fold(0us, |acc, &m| if in_range(m) { acc + 1us } else { acc })
}

#[cfg(test)]
mod parse_line_tests {
    use super::*;
    use std::io;

    // really dumb function that compares floating points by comparing their
    // string representations
    fn approx_eq_floats(num1: f64, num2: f64) -> bool {
        let num1_s = num1.to_string();
        let num2_s = num2.to_string();
        num1_s.as_slice() == num2_s.as_slice()
    }

    #[test]
    fn returns_float_result_for_valid_line() {
        let input_line: io::IoResult<String> = Ok("12.34".to_string());
        let end_marker = "foo";
        let parsed_line_result: Result<f64, &str> = parse_line(input_line, end_marker);
        assert!(parsed_line_result.is_ok());
        assert!(approx_eq_floats(12.34, parsed_line_result.unwrap()));

        let trimming_needed_line: io::IoResult<String> = Ok("    56.78  ".to_string());
        let trimmed_line_result = parse_line(trimming_needed_line, end_marker);
        assert!(trimmed_line_result.is_ok());
        assert!(approx_eq_floats(56.78, trimmed_line_result.unwrap()));
    }

    #[test]
    fn returns_err_for_ioerror_lines() {
        let err_input_line: io::IoResult<String> =
            Err(io::IoError { kind: io::OtherIoError, desc: "test", detail: None });
        let end_marker = "foo";
        assert!(parse_line(err_input_line, end_marker).is_err());
    }

    #[test]
    fn signals_end_of_file_with_end_marker() {
        let end_marker = "foo";
        let input_line: io::IoResult<String> = Ok("foo".to_string());
        assert_eq!(Err("eof"), parse_line(input_line, end_marker));
    }

    #[test]
    fn returns_err_for_non_float_lines() {
        let end_marker = "foo";
        let input_line: io::IoResult<String> = Ok("not a number".to_string());
        assert!(parse_line(input_line, end_marker).is_err());
    }

    #[test]
    fn returns_err_for_negative_measurements() {
        let end_marker = "foo";
        let input_line: io::IoResult<String> = Ok("-12.34".to_string());
        assert!(parse_line(input_line, end_marker).is_err());
    }
}

#[cfg(test)]
mod avg_float_vec_tests {
    use super::*;

    // really dumb function that compares floating points by comparing their
    // string representations
    fn approx_eq_floats(num1: f64, num2: f64) -> bool {
        let num1_s = num1.to_string();
        let num2_s = num2.to_string();
        num1_s.as_slice() == num2_s.as_slice()
    }

    #[test]
    fn returns_average() {
        let test_vec: Vec<f64> = vec![1.2, 3.4, 5.6, 7.8, 9.10, 11.12, 13.14];
        assert!(approx_eq_floats(7.3371428571, avg_float_vec(&test_vec)));
    }
}

#[cfg(test)]
mod num_within_range_tests {
    use super::*;

    #[test]
    fn one_num_within_range() {
        let test_vec: Vec<f64> = vec![1.2, 3.4, 5.6, 7.8, 9.10, 11.12, 13.14];
        assert_eq!(1, num_within_range(&test_vec, 0.0, 1.2));
        assert_eq!(7, num_within_range(&test_vec, 0.0, 15.0));
    }

    #[test]
    fn all_num_within_range() {
        let test_vec: Vec<f64> = vec![1.2, 3.4, 5.6, 7.8, 9.10, 11.12, 13.14];
        assert_eq!(test_vec.len(), num_within_range(&test_vec, 0.0, 15.0));
    }

    #[test]
    fn zero_when_none_in_range_positive() {
        let test_vec: Vec<f64> = vec![1.2, 3.4, 5.6, 7.8, 9.10, 11.12, 13.14];
        assert_eq!(0, num_within_range(&test_vec, 1000.0, 2000.0));
    }

    #[test]
    fn zero_when_none_in_range_negative() {
        let test_vec: Vec<f64> = vec![1.2, 3.4, 5.6, 7.8, 9.10, 11.12, 13.14];
        assert_eq!(0, num_within_range(&test_vec, -10.0, -1.0));
    }
}
