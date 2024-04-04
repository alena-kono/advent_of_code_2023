use advent_of_code_2023::cli_parser::Config;
use advent_of_code_2023::day_03;

#[test]
fn run_part_1() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_03.txt".to_string(),
    });
    assert_eq!(day_03::task_1::run(conf), 4361);
}

#[test]
fn run_part_1_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_03.txt".to_string(),
    });
    assert_eq!(day_03::task_1::run(conf), 553079);
}

#[test]
fn run_part_2() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_03.txt".to_string(),
    });
    assert_eq!(day_03::task_2::run(conf), 467835);
}

#[test]
fn run_part_2_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_03.txt".to_string(),
    });
    assert_eq!(day_03::task_2::run(conf), 84363105);
}
