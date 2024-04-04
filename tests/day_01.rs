use advent_of_code_2023::cli_parser::Config;
use advent_of_code_2023::day_01;

#[test]
fn run_part_1() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_01.txt".to_string(),
    });
    assert_eq!(day_01::task_1::run(conf), 142);
}

#[test]
fn run_part_1_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_01.txt".to_string(),
    });
    assert_eq!(day_01::task_1::run(conf), 56465);
}

#[test]
fn run_part_2() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_01_task_2.txt".to_string(),
    });
    assert_eq!(day_01::task_2::run(conf), 281);
}

#[test]
fn run_part_2_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_01.txt".to_string(),
    });
    assert_eq!(day_01::task_2::run(conf), 55902);
}
