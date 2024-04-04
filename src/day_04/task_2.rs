use crate::cli_parser::Config;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn run(config: Option<Config>) -> i32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");
    let result = calc_points(&input_data);
    println!("{}", &result);
    result
}

fn calc_points(s: &str) -> i32 {
    let mut cards_cnt: i32 = 0;
    let mut instances: HashMap<i32, i32> = HashMap::new();

    for line in s.lines() {
        if !line.is_empty() {
            let card = Scratchcard::from(&line);
            let mut modifier: i32 = 1;
            // Process original
            let cnt = instances.entry(card.id).or_insert(0);
            if *cnt > 0 {
                modifier = *cnt + 1;
            }
            *cnt += 1;
            cards_cnt += 1;
            // Process copies
            let pts = card.count_winning_actual_nums();
            if pts < 1 {
                continue;
            }
            for id in card.id + 1..card.id + pts + 1 {
                let cnt = instances.entry(id).or_insert(0);
                *cnt += modifier;
                cards_cnt += modifier;
            }
        }
    }
    cards_cnt
}

struct Scratchcard<'a> {
    pub id: i32,
    pub winning_nums: HashSet<&'a str>,
    pub actual_nums: HashSet<&'a str>,
}

impl Scratchcard<'_> {
    pub fn from(s: &str) -> Scratchcard {
        let card: Vec<&str> = s.split(":").collect::<Vec<&str>>();
        let card_id: i32 = card
            .get(0)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse()
            .unwrap();
        let nums: Vec<&str> = card.get(1).unwrap().split("|").collect();
        let win: HashSet<&str> = nums.get(0).unwrap().split_whitespace().collect();
        let act: HashSet<&str> = nums.get(1).unwrap().split_whitespace().collect();
        Scratchcard {
            id: card_id,
            winning_nums: win,
            actual_nums: act,
        }
    }

    pub fn count_winning_actual_nums(&self) -> i32 {
        self.winning_nums.intersection(&self.actual_nums).count() as i32
    }
}
