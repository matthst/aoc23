use std::collections::HashSet;
use std::fs::read_to_string;

mod util;

fn main() {
    let input: String = read_to_string("res/day04").unwrap();
    util::time_function("Part 1", input.clone(), part1);
    util::time_function("Part 2", input.clone(), part2);
}

fn part1(input: String) -> String {
    input
        .lines()
        .map(|l| match get_match_count(l) {
            0 => 0,
            n => 2_i32.pow((n - 1) as u32),
        })
        .sum::<i32>()
        .to_string()
}

fn part2(input: String) -> String {
    // (card count, wins)
    let mut cards: Vec<(i32, i32)> = input.lines().map(|l| (1, get_match_count(l))).collect();
    for i in 0..cards.len() {
        for j in i + 1..(i + 1 + cards[i].1 as usize) {
            cards[j].0 += cards[i].0
        }
    }
    cards.iter().map(|c| c.0).sum::<i32>().to_string()
}

fn get_match_count(line: &str) -> i32 {
    parse_nums(&line[10..40])
        .intersection(&parse_nums(&line[42..]))
        .count() as i32
}

fn parse_nums(line: &str) -> HashSet<i32> {
    line.trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect::<HashSet<i32>>()
}
