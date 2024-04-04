use regex;

use crate::cli_parser::Config;
use std::{collections::HashMap, fs};

pub fn run(config: Option<Config>) -> u32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");

    let result = calc_sum(&input_data);
    println!("{}", &result);
    result
}

fn calc_sum(input_data: &str) -> u32 {
    const DIGIT_BASE: u32 = 10;
    let spelled_out_digits = HashMap::from([
        ("one".to_string(), "1".to_string()),
        ("two".to_string(), "2".to_string()),
        ("three".to_string(), "3".to_string()),
        ("four".to_string(), "4".to_string()),
        ("five".to_string(), "5".to_string()),
        ("six".to_string(), "6".to_string()),
        ("seven".to_string(), "7".to_string()),
        ("eight".to_string(), "8".to_string()),
        ("nine".to_string(), "9".to_string()),
    ]);

    let re = regex::Regex::new(r"(?:[0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reversed_re =
        regex::Regex::new(r"(?:[0-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    let mut reversed_line: String;
    let mut digit1: Option<String>;
    let mut digit2: Option<String>;
    let mut merged_digit: String;
    let mut sum = 0;
    let mut result: String;

    for line in input_data.lines() {
        // First digit
        let regex_result = re.find(line);
        match regex_result {
            Some(m) => {
                result = m.as_str().to_string();
            }
            None => continue,
        }
        if result.chars().nth(0).unwrap().is_digit(DIGIT_BASE) {
            digit1 = Some(result.to_string());
        } else {
            digit1 = spelled_out_digits.get(&result).cloned();
        }

        // Last digit
        reversed_line = line.chars().rev().collect::<String>();
        let regex_result = reversed_re.find(&reversed_line);
        match regex_result {
            Some(m) => {
                result = m.as_str().to_string();
            }
            None => continue,
        }
        if result.chars().nth(0).unwrap().is_digit(DIGIT_BASE) {
            digit2 = Some(result.to_string());
        } else {
            digit2 = spelled_out_digits
                .get(&result.chars().rev().collect::<String>())
                .cloned();
        }

        // Check parsed digits
        if digit1.is_some() && digit2.is_none() {
            digit2 = digit1.clone();
        }
        // Merge digits into one
        merged_digit = format!("{}{}", &digit1.unwrap(), &digit2.unwrap());

        sum += merged_digit.parse::<u32>().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn calculate_sum() {
        let input_expected = HashMap::from([
            ("1peace2love3\ngreat91\nfinal2", 126),
            ("0peace2love3\ng2rea4t\nfina1l2", 39),
            ("peacelove\ngreat9\nfinal2", 121),
            ("peacelove\ngreat\nfinal", 0),
            ("", 0),
            ("empty", 0),
            ("\n", 0),
            ("first10", 10),
            ("111peace5l4o1ve2\ngreat0", 12),
            ("four2final1", 41),
            ("1four2final1", 11),
            ("one2six", 16),
            ("loveone2six0", 10),
            ("one2six\nlovegreatone", 27),
            ("pcg91vqrfpxxzzzoneightzt", 98),
        ]);

        for (input, expected) in input_expected.iter() {
            assert_eq!(calc_sum(input), *expected);
        }
    }
}
