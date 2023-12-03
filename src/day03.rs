use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

use itertools::Itertools;

mod util;

static SYMBOLS: [char; 10] = ['*', '/', '-', '@', '$', '=', '%', '+', '#', '&'];

fn main() {
    let input: String = read_to_string("res/day03").unwrap();
    util::time_function("Part 1", input.clone(), part1);
    util::time_function("Part 2", input.clone(), part2);
}

fn part1(input: String) -> String {
    let schematic = Schematic::new(input);
    let mut sum: u32 = 0;
    let mut y = 0;
    while y < schematic.rows {
        let mut x = 0;
        while x < schematic.columns {
            let (num, len) = schematic.validate_part_num(x, y).unwrap_or((0, 1));
            sum += num;
            x += len;
        }
        y += 1;
    }

    return sum.to_string();
}

fn part2(input: String) -> String {
    let schematic = Schematic::new(input);
    let sum = (0..schematic.rows)
        .cartesian_product(0..schematic.columns)
        .into_iter()
        .filter_map(|xy| schematic.get_gear_ratios(xy))
        .sum::<u32>();
    return sum.to_string();
}

struct Schematic {
    rows: usize,
    columns: usize,
    data: Vec<char>,
}

impl Schematic {
    pub fn new(input: String) -> Schematic {
        let mut data = input.chars().collect::<Vec<char>>();
        let columns = data.iter().position(|&c| c == '\n').unwrap();
        let rows = data.iter().filter(|&&c| c == '\n').count();
        data = data.into_iter().filter(|&c| c != '\n').collect();

        Schematic {
            rows,
            columns,
            data,
        }
    }

    fn validate_part_num(&self, x: usize, y: usize) -> Option<(u32, usize)> {
        let mut num = 0;
        let mut len: usize = 0;
        while x + len != self.rows && self[(x + len, y)].is_ascii_digit() {
            num = num * 10 + self[(x + len, y)].to_digit(10).unwrap();
            len += 1;
        }

        if len == 0 {
            return None;
        }

        if (x.saturating_sub(1)..(x + len + 1).min(self.columns))
            .cartesian_product(y.saturating_sub(1)..(y + 2).min(self.rows))
            .any(|(xs, ys)| SYMBOLS.contains(&self[(xs, ys)]))
        {
            return Some((num, len));
        }
        None
    }

    fn get_gear_ratios(&self, xy: (usize, usize)) -> Option<u32> {
        if self[xy] != '*' {
            return None;
        }

        let mut remaining_checks = (xy.0.saturating_sub(1)..(xy.0 + 2).min(self.columns))
            .cartesian_product(xy.1.saturating_sub(1)..(xy.1 + 2).min(self.rows))
            .collect::<Vec<(usize, usize)>>();
        let mut ratios: Vec<u32> = Vec::new();

        while let Some(xy_c) = remaining_checks.pop() {
            if !self[xy_c].is_ascii_digit() {
                continue;
            }

            let mut xstart = xy_c.0;
            let mut xend = xy_c.0;

            while xend != self.rows && self[(xend, xy_c.1)].is_ascii_digit() {
                xend += 1;
                try_remove_coords(&mut remaining_checks, (xend, xy_c.1));
            }

            while xstart != 0 && self[(xstart - 1, xy_c.1)].is_ascii_digit() {
                xstart -= 1;
                try_remove_coords(&mut remaining_checks, (xstart, xy_c.1));
            }

            let num: u32 = self.data
                [(xy_c.1 * self.columns + xstart)..(xy_c.1 * self.columns + xend)]
                .iter()
                .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap());
            ratios.push(num);
        }

        if ratios.len() == 2 {
            return Some(ratios.iter().product::<u32>());
        }
        None
    }
}

impl Index<(usize, usize)> for Schematic {
    type Output = char;

    fn index(&self, xy: (usize, usize)) -> &char {
        &self.data[xy.0 + xy.1 * self.columns]
    }
}

impl IndexMut<(usize, usize)> for Schematic {
    fn index_mut(&mut self, xy: (usize, usize)) -> &mut char {
        &mut self.data[xy.0 + xy.1 * self.columns]
    }
}

fn try_remove_coords(coords: &mut Vec<(usize, usize)>, xy: (usize, usize)) {
    if let Some(pos) = coords.iter().position(|&x| x == xy) {
        coords.swap_remove(pos);
    }
}
