use std::fs::read_to_string;
use std::iter::zip;

use regex::Regex;

mod util;

fn main() {
    let input: String = read_to_string("res/day06").unwrap();
    util::time_function("Part 1", input.clone(), part1);
    util::time_function("Part 2", input.clone(), part2);
}

fn part1(input: String) -> String {
    let mut lines = input.lines();
    let v1 = get_nums_from_str(lines.next().unwrap());
    let v2 = get_nums_from_str(lines.next().unwrap());
    let times: Vec<(i64, i64)> = zip(v1, v2).collect();

    times
        .into_iter()
        .map(|time| (1..time.0).filter(|&i| i * (time.0 - i) > time.1).count())
        .product::<usize>()
        .to_string()
}

fn part2(input: String) -> String {
    let nums = get_nums_from_str(&*input.replace(" ", ""));
    (0..nums[0])
        .filter(|&i| i * (nums[0] - i) > nums[1])
        .count()
        .to_string()
}

fn get_nums_from_str(input: &str) -> Vec<i64> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|mat| mat.as_str().parse::<i64>().unwrap())
        .collect()
}
