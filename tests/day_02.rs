use advent_of_code_2023::cli_parser::Config;
use advent_of_code_2023::day_02;

#[test]
fn run_part_1() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_02.txt".to_string(),
    });
    assert_eq!(day_02::task_1::run(conf), 8);
}

#[test]
fn run_part_1_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_02.txt".to_string(),
    });
    assert_eq!(day_02::task_1::run(conf), 2377);
}

#[test]
fn run_part_2() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/example/day_02.txt".to_string(),
    });
    assert_eq!(day_02::task_2::run(conf), 2286);
}

#[test]
fn run_part_2_slow() {
    let conf: Option<Config> = Some(Config {
        file_path: "tests/data/real/day_02.txt".to_string(),
    });
    assert_eq!(day_02::task_2::run(conf), 71220);
}
