use crate::cli_parser::Config;
use std::fs;

pub fn run(config: Option<Config>) -> u32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");

    let result = calc_sum(&input_data);
    println!("{}", &result);
    result
}

fn calc_sum(input_data: &str) -> u32 {
    const DIGIT_BASE: u32 = 10;

    let mut digit1: Option<String>;
    let mut digit2: Option<String>;
    let mut composed_digit: String;
    let mut sum: u32 = 0;

    for line in input_data.lines() {
        digit1 = None;
        digit2 = None;

        for char in line.chars() {
            if char.is_digit(DIGIT_BASE) {
                if digit1.is_none() {
                    digit1 = Some(char.to_string());
                } else {
                    digit2 = Some(char.to_string());
                }
            }
        }

        if digit1.is_some() && digit2.is_none() {
            digit2 = digit1.clone();
        }

        if digit1.is_some() && digit2.is_some() {
            composed_digit = format!(
                "{}{}",
                digit1.clone().unwrap_or_default(),
                digit2.clone().unwrap_or_default()
            );
            // Convert str digits to u32
            let digit: u32 = composed_digit.parse().unwrap();
            sum += digit;
        }
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
            ("\n", 0),
            ("first10", 10),
            ("111peace5l4o1ve2\ngreat0", 12),
        ]);

        for (input, expected) in input_expected.iter() {
            assert_eq!(calc_sum(input), *expected);
        }
    }
}
