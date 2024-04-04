use crate::cli_parser::Config;
use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

pub fn run(config: Option<Config>) -> i32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");

    let matrix = Matrix::from(&input_data)
        .expect("Input data has columns different lengths to build a Matrix");

    let result = find_sum_of_gear_ratios(matrix);
    println!("{}", &result);
    result
}

fn find_sum_of_gear_ratios(matrix: Matrix) -> i32 {
    sum_gear_ratios(&swap_digits_and_symbols(&parse_digits(matrix)))
}

fn parse_digits(matrix: Matrix) -> HashMap<DigitPos, Vec<Pos>> {
    const DIGIT_BASE: u32 = 10;

    let mut digit_symb: HashMap<DigitPos, Vec<Pos>> = HashMap::new();
    let mut cur_digit = "".to_string();
    let mut start_pos: Option<Pos> = None;
    let mut cur_adj_spec_chars: Vec<Pos> = Vec::new();

    for (i, row) in matrix.data.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            dbg!("----");
            dbg!(col);
            if col.is_digit(DIGIT_BASE) {
                if cur_digit.is_empty() && start_pos.is_none() {
                    start_pos = Some(Pos::from(i as i32, j as i32));
                }
                cur_digit.push_str(&col.to_string());
                dbg!(&cur_digit);

                if let Some(a) = get_adjacent_special_chars(&matrix, i, j, row.iter().len()) {
                    cur_adj_spec_chars.extend(a);
                }
            } else {
                dbg!(&cur_digit);
                if let Some(p) = start_pos {
                    cur_adj_spec_chars.dedup();
                    let cur_dig = cur_digit.parse::<i32>().unwrap();
                    digit_symb
                        .entry(
                            DigitPos::from(p, Pos::from(i as i32, j as i32 - 1), cur_dig).unwrap(),
                        )
                        .or_insert(cur_adj_spec_chars.clone());
                }
                start_pos = None;
                cur_digit = "".to_string();
                cur_adj_spec_chars = vec![];
            }
            if row.get(j + 1).is_none() {
                dbg!(&cur_digit);
                let mut jj: i32 = j as i32 - 1;
                if col.is_digit(DIGIT_BASE) {
                    jj = j as i32;
                }
                cur_adj_spec_chars.dedup();
                if let Some(p) = start_pos {
                    let cur_dig = cur_digit.parse::<i32>().unwrap();
                    digit_symb
                        .entry(DigitPos::from(p, Pos::from(i as i32, jj), cur_dig).unwrap())
                        .or_insert(cur_adj_spec_chars.clone());
                }
                start_pos = None;
                cur_digit = "".to_string();
                cur_adj_spec_chars = vec![];
            }
        }
    }
    digit_symb
}

fn swap_digits_and_symbols(m: &HashMap<DigitPos, Vec<Pos>>) -> HashMap<Pos, HashSet<DigitPos>> {
    let mut new: HashMap<Pos, HashSet<DigitPos>> = HashMap::new();

    for (dig, positions) in m.iter() {
        for pos in positions {
            let p = new.entry(pos.clone()).or_insert(HashSet::new());
            if !p.contains(dig) {
                p.insert(dig.clone());
            }
        }
    }
    new
}

fn sum_gear_ratios(m: &HashMap<Pos, HashSet<DigitPos>>) -> i32 {
    const ADJ_DIGITS_COUNT: i32 = 2;
    let mut sum: i32 = 0;

    for (_pos, digits) in m.iter() {
        if digits.len() as i32 == ADJ_DIGITS_COUNT {
            let mut ratios: i32 = 1;
            for d in digits {
                ratios *= d.value;
            }
            sum += ratios;
        }
    }
    sum
}

fn get_adjacent_special_chars(
    matrix: &Matrix,
    i: usize,
    j: usize,
    cols_len: usize,
) -> Option<Vec<Pos>> {
    let mut adjacent_chars: Vec<Pos> = Vec::new();
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

    for (ii, x) in matrix.data[i_start..i_end].iter().enumerate() {
        for (jj, y) in x[j_start..j_end].iter().enumerate() {
            if is_special_char(y, &excluded_chars) {
                adjacent_chars.push(Pos::from((ii + i_start) as i32, (jj + j_start) as i32));
            } else {
            }
        }
    }
    if adjacent_chars.is_empty() {
        return None;
    }
    Some(adjacent_chars)
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    pub i: i32,
    pub j: i32,
}

impl Pos {
    pub fn from(i: i32, j: i32) -> Pos {
        Pos { i, j }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct DigitPos {
    pub start: Pos,
    pub end: Pos,
    pub value: i32,
}

impl DigitPos {
    pub fn from(start: Pos, end: Pos, value: i32) -> Result<DigitPos, String> {
        if start.j > end.j || start.i != end.i {
            let err_msg = format!("Positions are invalid: start={:?}, end={:?}", start, end);
            return Err(err_msg);
        }
        Ok(DigitPos { start, end, value })
    }
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
    fn sum_gear_ratios_multiple() {
        let input: HashMap<Pos, HashSet<DigitPos>> = HashMap::from([
            (
                Pos { i: 1, j: 1 },
                HashSet::from([
                    DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 8).unwrap(),
                    DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 9).unwrap(),
                ]),
            ),
            (
                Pos { i: 2, j: 1 },
                HashSet::from([
                    DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 3).unwrap(),
                    DigitPos::from(Pos { i: 1, j: 4 }, Pos { i: 1, j: 4 }, 2).unwrap(),
                ]),
            ),
        ]);

        assert_eq!(sum_gear_ratios(&input), 78);
    }

    #[test]
    fn sum_gear_ratios_one() {
        let input: HashMap<Pos, HashSet<DigitPos>> = HashMap::from([
            (
                Pos { i: 1, j: 1 },
                HashSet::from([
                    DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 8).unwrap(),
                    DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 9).unwrap(),
                ]),
            ),
            (
                Pos { i: 2, j: 1 },
                HashSet::from([DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 3).unwrap()]),
            ),
        ]);

        assert_eq!(sum_gear_ratios(&input), 72);
    }

    #[test]
    fn sum_gear_ratios_zero() {
        let input: HashMap<Pos, HashSet<DigitPos>> = HashMap::from([
            (
                Pos { i: 1, j: 1 },
                HashSet::from([DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 8).unwrap()]),
            ),
            (
                Pos { i: 2, j: 1 },
                HashSet::from([DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 3).unwrap()]),
            ),
        ]);

        assert_eq!(sum_gear_ratios(&input), 0);
    }

    #[test]
    fn swap_hashmap_multiple() {
        let input: HashMap<DigitPos, Vec<Pos>> = HashMap::from([
            (
                DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 8).unwrap(),
                vec![Pos { i: 1, j: 1 }],
            ),
            (
                DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 9).unwrap(),
                vec![Pos { i: 1, j: 1 }, Pos { i: 2, j: 1 }],
            ),
        ]);
        let expected: HashMap<Pos, HashSet<DigitPos>> = HashMap::from([
            (
                Pos { i: 1, j: 1 },
                HashSet::from([
                    DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 8).unwrap(),
                    DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 9).unwrap(),
                ]),
            ),
            (
                Pos { i: 2, j: 1 },
                HashSet::from([DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 2 }, 9).unwrap()]),
            ),
        ]);

        assert_eq!(swap_digits_and_symbols(&input), expected);
    }

    #[test]
    fn swap_hashmap_one() {
        let input: HashMap<DigitPos, Vec<Pos>> = HashMap::from([(
            DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 8).unwrap(),
            vec![Pos { i: 1, j: 1 }],
        )]);
        let expected: HashMap<Pos, HashSet<DigitPos>> = HashMap::from([(
            Pos { i: 1, j: 1 },
            HashSet::from([DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 8).unwrap()]),
        )]);

        assert_eq!(swap_digits_and_symbols(&input), expected);
    }

    #[test]
    fn find_gears_tiny() {
        let input = String::from(
            "\
..1\n
.+.\n
...",
        );
        let matrix = Matrix::from(&input)
            .expect("Input data has columns different lengths to build a Matrix");
        let expected: HashMap<DigitPos, Vec<Pos>> = HashMap::from([(
            DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 1).unwrap(),
            vec![Pos { i: 1, j: 1 }],
        )]);
        assert_eq!(parse_digits(matrix), expected);
    }

    #[test]
    fn find_gears_valid() {
        let input = String::from(
            "\
..1\n
2+4\n
...",
        );
        let matrix = Matrix::from(&input)
            .expect("Input data has columns different lengths to build a Matrix");
        let expected: HashMap<DigitPos, Vec<Pos>> = HashMap::from([
            (
                DigitPos::from(Pos { i: 0, j: 2 }, Pos { i: 0, j: 2 }, 1).unwrap(),
                vec![Pos { i: 1, j: 1 }],
            ),
            (
                DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 0 }, 2).unwrap(),
                vec![Pos { i: 1, j: 1 }],
            ),
            (
                DigitPos::from(Pos { i: 1, j: 2 }, Pos { i: 1, j: 2 }, 4).unwrap(),
                vec![Pos { i: 1, j: 1 }],
            ),
        ]);
        assert_eq!(parse_digits(matrix), expected);
    }

    #[test]
    fn find_gears_valid_numbers() {
        let input = String::from(
            "\
.11\n
2+.\n
88.",
        );
        let matrix = Matrix::from(&input)
            .expect("Input data has columns different lengths to build a Matrix");
        let expected: HashMap<DigitPos, Vec<Pos>> = HashMap::from([
            (
                DigitPos::from(Pos { i: 0, j: 1 }, Pos { i: 0, j: 2 }, 11).unwrap(),
                vec![Pos { i: 1, j: 1 }],
            ),
            (
                DigitPos::from(Pos { i: 1, j: 0 }, Pos { i: 1, j: 0 }, 2).unwrap(),
                vec![Pos { i: 1, j: 1 }],
            ),
            (
                DigitPos::from(Pos { i: 2, j: 0 }, Pos { i: 2, j: 1 }, 88).unwrap(),
                vec![Pos { i: 1, j: 1 }],
            ),
        ]);
        assert_eq!(parse_digits(matrix), expected);
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

        assert_eq!(
            get_adjacent_special_chars(&matrix, 0, 0, 3),
            Some(vec![Pos { i: 1, j: 1 }])
        );
        assert_eq!(
            get_adjacent_special_chars(&matrix, 2, 2, 3),
            Some(vec![Pos { i: 1, j: 1 }])
        );
        assert_eq!(
            get_adjacent_special_chars(&matrix, 1, 2, 3),
            Some(vec![Pos { i: 1, j: 1 }])
        );
        assert_eq!(
            get_adjacent_special_chars(&matrix, 2, 1, 3),
            Some(vec![Pos { i: 1, j: 1 }])
        );
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

        assert_eq!(
            get_adjacent_special_chars(&matrix, 0, 5, 8),
            Some(vec![Pos { i: 0, j: 4 }])
        );
        assert_eq!(
            get_adjacent_special_chars(&matrix, 2, 2, 8),
            Some(vec![Pos { i: 1, j: 2 }, Pos { i: 2, j: 1 }])
        );
        assert_eq!(
            get_adjacent_special_chars(&matrix, 1, 0, 8),
            Some(vec![Pos { i: 2, j: 1 }])
        );
        assert_eq!(
            get_adjacent_special_chars(&matrix, 2, 3, 8),
            Some(vec![Pos { i: 1, j: 2 }])
        );
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

        assert_eq!(
            get_adjacent_special_chars(&matrix, 1, 1, 8),
            Some(vec![Pos { i: 0, j: 1 }])
        );
        assert_eq!(get_adjacent_special_chars(&matrix, 1, 4, 8), None);
        assert_eq!(
            get_adjacent_special_chars(&matrix, 1, 7, 8),
            Some(vec![Pos { i: 2, j: 7 }])
        );
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

        assert_eq!(
            get_adjacent_special_chars(&matrix, 2, 0, 3),
            Some(vec![Pos { i: 1, j: 1 }])
        );
        assert_eq!(
            get_adjacent_special_chars(&matrix, 2, 2, 3),
            Some(vec![Pos { i: 1, j: 1 }])
        );
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

        assert_eq!(get_adjacent_special_chars(&matrix, 0, 4, 5), None);
        assert_eq!(get_adjacent_special_chars(&matrix, 1, 3, 5), None);
        assert_eq!(get_adjacent_special_chars(&matrix, 2, 3, 5), None);
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
