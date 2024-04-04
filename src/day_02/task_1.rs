use crate::cli_parser::Config;
use std::fs;

pub fn run(config: Option<Config>) -> i32 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");

    let bag: Vec<GameSet> = vec![GameSet::from(&vec![
        Cube::from("green".to_string(), 13),
        Cube::from("red".to_string(), 12),
        Cube::from("blue".to_string(), 14),
    ])];
    let result = sum_possible_game_ids(&input_data, &bag);
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

    pub fn is_possible_within(&self, bag: &Vec<GameSet>) -> bool {
        // Unoptimized impl
        for big_set in bag {
            for big_cube in &big_set.cubes {
                for small_set in &self.bag {
                    for small_cube in &small_set.cubes {
                        if small_cube.colour != big_cube.colour {
                            continue;
                        }
                        if big_cube.value - small_cube.value < 0 {
                            return false;
                        }
                    }
                }
            }
        }
        true
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

fn sum_possible_game_ids(input_data: &str, bag: &Vec<GameSet>) -> i32 {
    let mut sum = 0;

    for line in input_data.lines() {
        if !line.contains("Game") {
            continue;
        }
        let game = parse_game(line);

        // println!("{:?} - {}", &game, &game.is_possible_within(&bag));

        if game.is_possible_within(&bag) {
            sum += game.id;
        }
    }
    sum
}

// fn is_game_possible(big_bag: &Bag, small_bag: &Bag) -> bool {
//     for (colour, value) in big_bag.iter() {
//         let small_value = small_bag.get(colour);
//         match small_value {
//             Some(v) => {
//                 if value - v < 0 {
//                     return false;
//                 }
//             }
//             None => {
//                 return false;
//             }
//         }
//     }
//     true
// }

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
    fn calc_possible_game_ids_one_ok() {
        let input = "\
Game 1: 1 green, 1 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 1 red; 1 green, 2 blue, 1 red; 3 blue, 3 green\n
Game 2: 20 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green\n";
        let bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 10),
            Cube::from("red".to_string(), 7),
            Cube::from("blue".to_string(), 8),
        ])];

        assert_eq!(sum_possible_game_ids(&input, &bag), 1);
    }

    #[test]
    fn calc_possible_game_ids_both_ok() {
        let input = "\
Game 1: 1 green, 1 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 1 red; 1 green, 2 blue, 1 red; 3 blue, 3 green\n
Game 2: 20 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green\n";
        let bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 21),
            Cube::from("red".to_string(), 7),
            Cube::from("blue".to_string(), 8),
        ])];

        assert_eq!(sum_possible_game_ids(&input, &bag), 3);
    }

    #[test]
    fn calc_possible_game_ids_both_not_ok() {
        let input = "\
Game 1: 1 green, 1 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 1 red; 1 green, 2 blue, 1 red; 3 blue, 3 green\n
Game 2: 20 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green\n";
        let bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 1),
            Cube::from("red".to_string(), 2),
            Cube::from("blue".to_string(), 3),
        ])];

        assert_eq!(sum_possible_game_ids(&input, &bag), 0);
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

        // parse_game(&input);
        assert_eq!(parse_game(&input), expected);
    }

    #[test]
    fn is_game_possible_true() {
        let big_bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 1),
            Cube::from("red".to_string(), 2),
            Cube::from("blue".to_string(), 3),
        ])];

        let small_bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 1),
            Cube::from("red".to_string(), 1),
            Cube::from("blue".to_string(), 3),
        ])];
        let game = Game::from(1, small_bag);

        assert!(game.is_possible_within(&big_bag));
    }

    #[test]
    fn is_game_possible_false() {
        let big_bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 1),
            Cube::from("red".to_string(), 2),
            Cube::from("blue".to_string(), 3),
        ])];

        let small_bag: Vec<GameSet> = vec![GameSet::from(&vec![
            Cube::from("green".to_string(), 11),
            Cube::from("red".to_string(), 2),
            Cube::from("blue".to_string(), 1),
        ])];
        let game = Game::from(1, small_bag);

        assert!(!game.is_possible_within(&big_bag));
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn calc_possible_game_ids_one_ok() {
//         let input = "\
// Game 1: 1 green, 1 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 1 red; 1 green, 2 blue, 1 red; 3 blue, 3 green\n
// Game 2: 20 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green\n";
//         let bag: Bag = HashMap::from([
//             ("green".to_string(), 15),
//             ("blue".to_string(), 10),
//             ("red".to_string(), 10),
//         ]);
//
//         assert_eq!(sum_possible_game_ids(&input, &bag), 1);
//     }
//
//     #[test]
//     fn calc_possible_game_ids_both_ok() {
//         let input = "\
// Game 1: 1 green, 1 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 1 red; 1 green, 2 blue, 1 red; 3 blue, 3 green\n
// Game 2: 20 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green\n";
//         let bag: Bag = HashMap::from([
//             ("green".to_string(), 100),
//             ("blue".to_string(), 50),
//             ("red".to_string(), 50),
//         ]);
//
//         assert_eq!(sum_possible_game_ids(&input, &bag), 3);
//     }
//
//     #[test]
//     fn calc_possible_game_ids_both_not_ok() {
//         let input = "\
// Game 1: 1 green, 1 blue, 1 red; 2 green, 1 blue, 1 red; 1 green, 1 blue, 1 red; 1 green, 2 blue, 1 red; 3 blue, 3 green\n
// Game 2: 20 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green\n";
//         let bag: Bag = HashMap::from([
//             ("green".to_string(), 2),
//             ("blue".to_string(), 1),
//             ("red".to_string(), 3),
//         ]);
//
//         assert_eq!(sum_possible_game_ids(&input, &bag), 0);
//     }
//
//     #[test]
//     fn parse_line_valid() {
//         let input = String::from("Game 1: 1 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green
// ");
//         let expected = Game {
//             id: 1,
//             bag: HashMap::from([
//                 ("green".to_string(), 15),
//                 ("blue".to_string(), 10),
//                 ("red".to_string(), 4),
//             ]),
//         };
//
//         parse_game(&input);
//         assert_eq!(parse_game(&input), expected);
//     }
//
//     #[test]
//     fn is_game_possible_true() {
//         let big_bag = HashMap::from([
//             ("green".to_string(), 5),
//             ("blue".to_string(), 3),
//             ("red".to_string(), 1),
//         ]);
//
//         let small_bag = HashMap::from([
//             ("green".to_string(), 1),
//             ("blue".to_string(), 1),
//             ("red".to_string(), 1),
//         ]);
//
//         assert!(is_game_possible(&big_bag, &small_bag));
//     }
//
//     #[test]
//     fn is_game_possible_false() {
//         let big_bag = HashMap::from([
//             ("green".to_string(), 5),
//             ("blue".to_string(), 3),
//             ("red".to_string(), 1),
//         ]);
//
//         let small_bag = HashMap::from([
//             ("green".to_string(), 6),
//             ("blue".to_string(), 1),
//             ("red".to_string(), 1),
//         ]);
//
//         assert!(!is_game_possible(&big_bag, &small_bag));
//     }
// }
