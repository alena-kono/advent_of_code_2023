use crate::cli_parser::Config;
use std::{collections::HashMap, fs};

pub fn run(config: Option<Config>) -> i32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");

    let result = find_sum_of_powers_of_fewest_cubes(&input_data);
    println!("{}", &result);
    result
}

#[derive(Debug, PartialEq)]
struct Game {
    pub id: i32,
    pub bag: Vec<GameSet>,
}

impl Game {
    pub fn from(id: i32, bag: Vec<GameSet>) -> Game {
        Game { id, bag }
    }

    fn max_cubes_per_colour(&self) -> HashMap<String, i32> {
        let mut cubes: HashMap<String, i32> = HashMap::new();

        for set in &self.bag {
            for cube in &set.cubes {
                let value = cubes.entry(cube.colour.clone()).or_insert(0);
                if *value < cube.value {
                    *value = cube.value;
                }
            }
        }
        cubes
    }
}

#[derive(Debug, PartialEq)]
struct GameSet {
    pub cubes: Vec<Cube>,
}

impl GameSet {
    pub fn from(cubes: &Vec<Cube>) -> GameSet {
        GameSet {
            cubes: cubes.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cube {
    pub colour: String,
    pub value: i32,
}

impl Cube {
    pub fn from(colour: String, value: i32) -> Cube {
        Cube { colour, value }
    }
}

fn find_sum_of_powers_of_fewest_cubes(input_data: &str) -> i32 {
    let mut power: i32;
    let mut sum_of_powers = 0;

    for line in input_data.lines() {
        if !line.contains("Game") {
            continue;
        }
        let game = parse_game(line);

        power = 1;

        for (_, value) in game.max_cubes_per_colour().iter() {
            power *= value;
        }
        sum_of_powers += power;
    }
    sum_of_powers
}

fn parse_game(s: &str) -> Game {
    const PARTS_COUNT: usize = 2;
    let parts = s.split(':').collect::<Vec<&str>>();
    if parts.len() != PARTS_COUNT {
        panic!("parts len should be {PARTS_COUNT}");
    }

    let game_id: i32 = parts[0].replace("Game ", "").parse::<i32>().unwrap();
    let mut bag: Vec<GameSet> = Vec::new();

    for set in parts[1].split(';') {
        let mut cubes: Vec<Cube> = Vec::new();

        for cube in set.split(',') {
            let cube_data = cube.split_whitespace().collect::<Vec<&str>>();
            let value = cube_data[0].parse::<i32>().unwrap();
            let colour = cube_data[1].to_owned();
            cubes.push(Cube::from(colour, value));
        }
        bag.push(GameSet::from(&cubes));
    }
    Game::from(game_id, bag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_sum_of_powers_of_fewest_cubes_mutiple_games() {
        let input = "\
Game 1: 1 green, 1 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 1 red\n
Game 2: 20 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green\n
Game 3: 3 green, 2 blue";

        assert_eq!(
            find_sum_of_powers_of_fewest_cubes(&input),
            (2 * 1 * 1) + (20 * 3 * 1) + (3 * 2)
        );
    }

    #[test]
    fn find_sum_of_powers_of_fewest_cubes_one_game() {
        let input = "\
Game 1: 1 green, 3 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 10 red\n";

        assert_eq!(find_sum_of_powers_of_fewest_cubes(&input), (3 * 2 * 10));
    }

    #[test]
    fn parse_game_valid() {
        let input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let bag: Vec<GameSet> = vec![
            GameSet::from(&vec![
                Cube::from("blue".to_string(), 3),
                Cube::from("red".to_string(), 4),
            ]),
            GameSet::from(&vec![
                Cube::from("red".to_string(), 1),
                Cube::from("green".to_string(), 2),
                Cube::from("blue".to_string(), 6),
            ]),
            GameSet::from(&vec![Cube::from("green".to_string(), 2)]),
        ];
        let expected = Game { id: 1, bag };

        assert_eq!(parse_game(&input), expected);
    }

    #[test]
    fn max_cubes_per_colour_one_gameset() {
        let bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 1),
            Cube::from("red".to_string(), 2),
            Cube::from("blue".to_string(), 3),
        ])];
        let game = Game::from(1, bag);

        assert_eq!(
            game.max_cubes_per_colour(),
            HashMap::from([
                ("green".to_string(), 1),
                ("red".to_string(), 2),
                ("blue".to_string(), 3),
            ])
        );
    }

    #[test]
    fn max_cubes_per_colour_multiple_gamesets() {
        let bag: Vec<GameSet> = vec![
            GameSet::from(&vec![
                Cube::from("green".to_string(), 10),
                Cube::from("red".to_string(), 2),
                Cube::from("blue".to_string(), 3),
            ]),
            GameSet::from(&vec![
                Cube::from("green".to_string(), 1),
                Cube::from("red".to_string(), 20),
                Cube::from("blue".to_string(), 3),
            ]),
        ];
        let game = Game::from(1, bag);

        assert_eq!(
            game.max_cubes_per_colour(),
            HashMap::from([
                ("green".to_string(), 10),
                ("red".to_string(), 20),
                ("blue".to_string(), 3),
            ])
        );
    }

    #[test]
    fn max_cubes_per_colour_diff_multiple_gamesets() {
        let bag: Vec<GameSet> = vec![
            GameSet::from(&vec![Cube::from("red".to_string(), 8)]),
            GameSet::from(&vec![
                Cube::from("green".to_string(), 1),
                Cube::from("red".to_string(), 2),
                Cube::from("blue".to_string(), 3),
            ]),
        ];
        let game = Game::from(1, bag);

        assert_eq!(
            game.max_cubes_per_colour(),
            HashMap::from([
                ("green".to_string(), 1),
                ("red".to_string(), 8),
                ("blue".to_string(), 3),
            ])
        );
    }
}
