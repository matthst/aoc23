use std::cmp::{max, min};
use std::fs::read_to_string;

mod util;

fn main() {
    let input: String = read_to_string("res/day05").unwrap();
    util::time_function("Part 1", input.clone(), part1);
    util::time_function("Part 2", input.clone(), part2);
}

fn part1(input: String) -> String {
    let (seeds, maps) = parse_input(input);

    seeds
        .into_iter()
        .map(|s| maps.iter().fold(s, |res, m| m.apply_map_to_num(res)))
        .min()
        .unwrap()
        .to_string()
}

fn part2(input: String) -> String {
    let (seeds, maps) = parse_input(input);
    let mut ranges: Vec<(i64, i64)> = seeds.chunks(2).map(|a| (a[0], a[0] + a[1] - 1)).collect();

    for m in maps {
        ranges = ranges
            .iter()
            .map(|r| m.apply_map_to_range(r))
            .flatten()
            .collect()
    }

    ranges.iter().map(|r| r.0).min().unwrap().to_string()
}

fn parse_input(input: String) -> (Vec<i64>, Vec<Map>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let seeds: Vec<i64> = parts[0]
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let maps = parts
        .iter()
        .skip(1)
        .map(|p| Map::new(p.lines().skip(1).collect()))
        .collect::<Vec<Map>>();
    (seeds, maps)
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    pub fn new(input: Vec<&str>) -> Map {
        Map {
            ranges: input.iter().map(|s| Range::new(s)).collect(),
        }
    }

    fn apply_map_to_num(&self, seed: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|r| {
                if seed >= r.start && seed <= r.end {
                    return Some(seed + r.offset);
                }
                None
            })
            .unwrap_or(seed)
    }

    fn apply_map_to_range(&self, range: &(i64, i64)) -> Vec<(i64, i64)> {
        let (mut src, mut tmp, mut dst) = (vec![*range], vec![], vec![]);

        for map_r in &self.ranges {
            while let Some((start, end)) = src.pop() {
                if end < map_r.start || start > map_r.end {
                    tmp.push((start, end));
                    continue;
                }

                if start < map_r.start {
                    tmp.push((start, map_r.start - 1))
                }

                if end > map_r.end {
                    tmp.push((map_r.end + 1, end))
                }

                dst.push((
                    max(start, map_r.start) + map_r.offset,
                    min(end, map_r.end) + map_r.offset,
                ))
            }
            std::mem::swap(&mut src, &mut tmp)
        }
        dst.append(&mut src);
        dst
    }
}

struct Range {
    start: i64,
    end: i64,
    offset: i64,
}

impl Range {
    pub fn new(input: &str) -> Range {
        let nums = input
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Range {
            start: nums[1],
            end: nums[1] + nums[2] - 1,
            offset: nums[0] - nums[1],
        }
    }
}
