use std::collections::HashMap;
use std::fs::read_to_string;

mod util;

fn main() {
    let input: String = read_to_string("res/day01").unwrap();
    util::time_function("Part 1", input.clone(), part1);
    util::time_function("Part 2", input.clone(), part2);
}

fn part1(input: String) -> String {
    input
        .lines()
        .map(|s| {
            let digits: Vec<u32> = String::from(s)
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: String) -> String {
    let mut map = HashMap::new();
    map.insert("one", "o1e");
    map.insert("two", "t2e");
    map.insert("three", "t3e");
    map.insert("four", "4");
    map.insert("five", "f5e");
    map.insert("six", "6");
    map.insert("seven", "s7n");
    map.insert("eight", "e8t");
    map.insert("nine", "n9e");

    return part1(map.iter().fold(input, |acc, (k, v)| acc.replace(k, v)));
}
