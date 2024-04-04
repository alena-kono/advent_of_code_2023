use advent_of_code_2023::cli_parser::Config;
use advent_of_code_2023::day_04;

#[test]
fn run_part_1() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_04.txt".to_string(),
    });
    assert_eq!(day_04::task_1::run(conf), 13);
}

#[test]
fn run_part_1_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_04.txt".to_string(),
    });
    assert_eq!(day_04::task_1::run(conf), 32001);
}

#[test]
fn run_part_2() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_04.txt".to_string(),
    });
    assert_eq!(day_04::task_2::run(conf), 30);
}

#[test]
fn run_part_2_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_04.txt".to_string(),
    });
    assert_eq!(day_04::task_2::run(conf), 5037841);
}
