use advent_of_code_2023::cli_parser::Config;
use advent_of_code_2023::day_05;

#[test]
fn run_part_1() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_05.txt".to_string(),
    });
    assert_eq!(day_05::task_1::run(conf), 35);
}

#[test]
fn run_part_1_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_05.txt".to_string(),
    });
    assert_eq!(day_05::task_1::run(conf), 993500720);
}

#[test]
fn run_part_2() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_05.txt".to_string(),
    });
    assert_eq!(day_05::task_2::run(conf), 46);
}

#[test]
fn run_part_2_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_05.txt".to_string(),
    });
    assert_eq!(day_05::task_2::run(conf), 4917124);
}
