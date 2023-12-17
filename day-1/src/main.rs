use regex::Regex;
use std::{fs, str::Lines};

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
    let re = Regex::new(r"[0-9]+").unwrap();
    let total_sum: u32 = lines.fold(0, |total, line| {
        let line_numbers_list: Vec<String> = re
            .captures_iter(line)
            .map(|caps: regex::Captures<'_>| {
                let (match_value, []) = caps.extract();

                match_value.to_string()
            })
            .collect();

        let line_number = line_numbers_list.join("");
        let mut line_number_chars = line_number.chars();
        let first_digit = line_number_chars.next().unwrap();
        let last_digit = line_number_chars.next_back().unwrap_or(first_digit.clone());

        total
            + format!("{}{}", first_digit, last_digit)
                .parse()
                .unwrap_or(0)
    });

    total_sum
}
