use regex::{Regex, RegexSet};
use std::{collections::HashMap, fs, str::Lines};

fn main() {
    let input_str = fs::read_to_string("input.txt");

    match input_str {
        Ok(content) => {
            let lines: Lines = content.lines();

            let sum_of_calibration_values = sum_of_calibration_values_from_lines(lines.clone());

            println!(
                "Sum of calibration numbers =  {}",
                sum_of_calibration_values
            );
        }
        Err(err) => println!("Error: {:?}", err),
    }
}

fn sum_of_calibration_values_from_lines(lines: Lines) -> u32 {
    let regex_set = RegexSet::new([
        "[0-9]", r"one", r"two", r"three", r"four", r"five", r"six", r"seven", r"eight", r"nine",
    ])
    .unwrap();

    lines
        .map(|line| get_calibration_value_from_line(String::from(line), &regex_set))
        .sum()
}

/// It gets the calibration value, which is the first and the last numeric character from a line.
fn get_calibration_value_from_line(line: String, regex_set: &RegexSet) -> u32 {
    let mut hash_matches: HashMap<usize, String> = HashMap::new();
    let mut first_digit_position: Option<usize> = None;
    let mut last_digit_position: Option<usize> = None;

    for pattern in regex_set.patterns().iter() {
        let regex = Regex::new(&pattern).unwrap();

        for m in regex.find_iter(&line) {
            let value = m.as_str();
            let position = m.start();

            if position < *first_digit_position.get_or_insert(position) {
                first_digit_position = Some(position)
            }

            if position > *last_digit_position.get_or_insert(position) {
                last_digit_position = Some(position)
            }

            hash_matches.insert(position, String::from(value));
        }
    }

    if first_digit_position.is_none() {
        return 0;
    }

    let first_digit = hash_matches.get(&first_digit_position.unwrap()).unwrap();
    // When there is only one digit, then both digits will be the same
    // ```
    //  let first_digit = 1;
    //  let last_digit = None;
    //
    //  return 11;
    // ``
    let last_digit = if line.len() == 1 {
        first_digit
    } else {
        hash_matches.get(&last_digit_position.unwrap()).unwrap()
    };

    format!(
        "{}{}",
        sanitize_number(first_digit.to_string()),
        sanitize_number(last_digit.to_string())
    )
    .parse()
    .unwrap_or(0)
}

fn sanitize_number(str_number: String) -> String {
    match str_number.as_str() {
        "one" => String::from("1"),
        "two" => String::from("2"),
        "three" => String::from("3"),
        "four" => String::from("4"),
        "five" => String::from("5"),
        "six" => String::from("6"),
        "seven" => String::from("7"),
        "eight" => String::from("8"),
        "nine" => String::from("9"),
        rest => String::from(rest),
    }
}
