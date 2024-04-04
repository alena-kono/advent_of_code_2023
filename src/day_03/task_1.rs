use crate::cli_parser::Config;
use std::fs;

pub fn run(config: Option<Config>) -> i32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");

    let matrix = Matrix::from(&input_data)
        .expect("Input data has columns different lengths to build a Matrix");

    let result = sum_adjacent_nums(matrix);
    println!("{}", &result);
    result
}

fn sum_adjacent_nums(matrix: Matrix) -> i32 {
    const DIGIT_BASE: u32 = 10;
    const EMPTY: char = '.';

    let mut cur_digit_is_adj;
    let mut cur_digit;
    let mut sum: i32 = 0;

    for (i, row) in matrix.data.iter().enumerate() {
        cur_digit = "".to_string();
        cur_digit_is_adj = false;

        for (j, col) in row.iter().enumerate() {
            if col == &EMPTY || !col.is_digit(DIGIT_BASE) {
                if cur_digit_is_adj {
                    sum += cur_digit.parse::<i32>().unwrap();
                }
                cur_digit = "".to_string();
                cur_digit_is_adj = false;
            } else {
                cur_digit.push_str(&col.to_string());
                if is_adjacent_to_special_char(&matrix, i, j, row.iter().len()) {
                    cur_digit_is_adj = true;
                }
            }
        }
        if cur_digit_is_adj {
            sum += cur_digit.parse::<i32>().unwrap();
        }
    }
    sum
}

fn is_adjacent_to_special_char(matrix: &Matrix, i: usize, j: usize, cols_len: usize) -> bool {
    let excluded_chars: Vec<char> = vec!['.'];

    let i_start: usize;
    let j_start: usize;
    let i_end: usize;
    let j_end: usize;

    if i as i32 - 1 < 0 {
        i_start = 0;
    } else {
        i_start = i - 1;
    }
    if i + 1 > matrix.data.len() - 1 {
        i_end = matrix.data.len();
    } else {
        i_end = i + 2;
    }
    if j as i32 - 1 < 0 {
        j_start = 0;
    } else {
        j_start = j - 1;
    }
    if j + 2 > cols_len - 1 {
        j_end = cols_len;
    } else {
        j_end = j + 2;
    }

    for x in &matrix.data[i_start..i_end] {
        for y in &x[j_start..j_end] {
            if is_special_char(y, &excluded_chars) {
                return true;
            }
        }
    }
    false
}

fn is_special_char(c: &char, exclude: &[char]) -> bool {
    const DIGIT_BASE: u32 = 10;

    if exclude.contains(&c) {
        return false;
    } else if c.is_digit(DIGIT_BASE) && c.is_alphanumeric() {
        return false;
    }
    true
}

#[derive(Debug)]
struct Matrix {
    pub data: Vec<Vec<char>>,
}

impl Matrix {
    pub fn from(s: &str) -> Result<Matrix, &str> {
        let mut curr_len: usize;
        let mut prev_len: usize = 0 as usize;

        let mut data = Vec::new();
        let mut col_data = Vec::new();

        for row in s.lines() {
            curr_len = row.len();
            if row.is_empty() {
                continue;
            }
            if curr_len != prev_len && prev_len != 0 {
                return Err("Input data has columns different lengths to build a Matrix");
            }

            for col in row.chars() {
                col_data.push(col)
            }
            data.push(col_data.clone());
            col_data.clear();
            prev_len = curr_len;
        }
        Ok(Matrix { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn sum_adjacent_nums_valid() {
        let input_expected: HashMap<&str, i32> = HashMap::from([
            (
                "\
..1\n
2+4\n
...",
                7,
            ),
            (
                "\
*.1\n
.2.\n
...",
                2,
            ),
            (
                "\
*11\n
.2.\n
...",
                13,
            ),
            (
                "\
1..\n
...\n
...",
                0,
            ),
            (
                "\
...\n
...\n
..1",
                0,
            ),
            (
                "\
...\n
...\n
...",
                0,
            ),
            (
                "\
...\n
+..\n
...",
                0,
            ),
            (
                "\
1..\n
.+.\n
...",
                1,
            ),
            (
                "\
...\n
.+.\n
..1",
                1,
            ),
            (
                "\
123\n
.+.\n
..1",
                124,
            ),
            (
                "\
/..\n
.3.\n
..1",
                3,
            ),
            (
                "\
1..\n
2+.\n
3.1",
                7,
            ),
            (
                "\
456\n
123\n
789",
                0,
            ),
            (
                "\
..11@20.\n
5.$.....\n
.&10000.",
                10036,
            ),
            (
                "\
...1?.0.\n
5.$..5..\n
.&.2.0..",
                13,
            ),
            (
                "\
......\n
..81..\n
.&....",
                81,
            ),
            (
                "\
......\n
.1234.\n
.....+",
                1234,
            ),
            (
                "\
.+......\n
.1..5..1\n
.......-",
                2,
            ),
        ]);

        for (inp, exp) in input_expected.iter() {
            let matrix = Matrix::from(&inp)
                .expect("Input data has columns different lengths to build a Matrix");
            assert_eq!(sum_adjacent_nums(matrix), *exp);
        }
    }

    #[test]
    fn is_adjacent_to_special_char_zero_true() {
        let s = String::from(
            "\
..1\n
2+4\n
...",
        );
        let matrix =
            Matrix::from(&s).expect("Input data has columns different lengths to build a Matrix");

        assert_eq!(is_adjacent_to_special_char(&matrix, 0, 0, 3), true);
        assert_eq!(is_adjacent_to_special_char(&matrix, 2, 2, 3), true);
        assert_eq!(is_adjacent_to_special_char(&matrix, 1, 2, 3), true);
        assert_eq!(is_adjacent_to_special_char(&matrix, 2, 1, 3), true);
    }

    #[test]
    fn is_adjacent_to_special_char_hard_case_true() {
        let s = String::from(
            "\
..11@20.\n
5.$.....\n
.&10000.",
        );
        let matrix =
            Matrix::from(&s).expect("Input data has columns different lengths to build a Matrix");

        assert_eq!(is_adjacent_to_special_char(&matrix, 0, 5, 8), true);
        assert_eq!(is_adjacent_to_special_char(&matrix, 2, 2, 8), true);
        assert_eq!(is_adjacent_to_special_char(&matrix, 1, 0, 8), true);
        assert_eq!(is_adjacent_to_special_char(&matrix, 2, 3, 8), true);
    }

    #[test]
    fn is_adjacent_to_special_char_after_adjacent() {
        let s = String::from(
            "\
.+......\n
.1..5..1\n
.......-",
        );
        let matrix =
            Matrix::from(&s).expect("Input data has columns different lengths to build a Matrix");

        // assert_eq!(is_adjacent_to_special_char(&matrix, 1, 1, 8), false);
        // assert_eq!(is_adjacent_to_special_char(&matrix, 1, 4, 8), false);
        assert_eq!(is_adjacent_to_special_char(&matrix, 1, 7, 8), true);
    }

    #[test]
    fn is_adjacent_to_special_char_last_line_true() {
        let s = String::from(
            "\
1..\n
2+.\n
3.1",
        );
        let matrix =
            Matrix::from(&s).expect("Input data has columns different lengths to build a Matrix");

        assert_eq!(is_adjacent_to_special_char(&matrix, 2, 0, 3), true);
    }

    #[test]
    fn is_adjacent_to_special_char_zero_false() {
        let s = String::from(
            "\
..1.2\n
3+4..\n
*..8.",
        );
        let matrix =
            Matrix::from(&s).expect("Input data has columns different lengths to build a Matrix");

        assert_eq!(is_adjacent_to_special_char(&matrix, 0, 4, 5), false);
        assert_eq!(is_adjacent_to_special_char(&matrix, 1, 3, 5), false);
        assert_eq!(is_adjacent_to_special_char(&matrix, 2, 3, 5), false);
    }

    #[test]
    fn is_special_char_true() {
        let chars: Vec<char> = vec!['a', 'B', '!', '?', '*', '+', '-', '%', '&', '.'];
        let exclude: Vec<char> = vec![];

        for c in chars {
            assert_eq!(is_special_char(&c, &exclude), true);
        }
    }

    #[test]
    fn is_special_char_false() {
        let chars: Vec<char> = vec!['1', '.'];
        let exclude: Vec<char> = vec!['.'];

        for c in chars {
            assert_eq!(is_special_char(&c, &exclude), false);
        }
    }
}
