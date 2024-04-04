use crate::cli_parser::Config;
use std::{collections::HashSet, fs};

pub fn run(config: Option<Config>) -> i32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");
    let result = calc_points(&input_data);
    println!("{}", &result);
    result
}

fn calc_points(s: &str) -> i32 {
    let mut points: i32 = 0;
    let mut cur_p: i32;

    for line in s.lines() {
        if !line.is_empty() {
            cur_p = Scratchcard::from(&line).count_winning_actual_nums();
            if cur_p > 0 {
                points += 2_i32.pow(cur_p as u32 - 1);
            }
        }
    }
    points
}

struct Scratchcard<'a> {
    pub winning_nums: HashSet<&'a str>,
    pub actual_nums: HashSet<&'a str>,
}

impl Scratchcard<'_> {
    pub fn from(s: &str) -> Scratchcard {
        let card: Vec<&str> = s.split(":").collect::<Vec<&str>>();
        let nums: Vec<&str> = card.get(1).unwrap().split("|").collect();
        let win: HashSet<&str> = nums.get(0).unwrap().split_whitespace().collect();
        let act: HashSet<&str> = nums.get(1).unwrap().split_whitespace().collect();
        Scratchcard {
            winning_nums: win,
            actual_nums: act,
        }
    }

    pub fn count_winning_actual_nums(&self) -> i32 {
        self.winning_nums.intersection(&self.actual_nums).count() as i32
    }
}
