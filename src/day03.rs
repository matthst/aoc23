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

        let x_range = x.saturating_sub(1)..(x + len + 1).min(self.columns);
        let y_range = y.saturating_sub(1)..(y + 2).min(self.rows);
        if x_range
            .cartesian_product(y_range)
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

        let (x, y) = xy;
        let x_range = x.saturating_sub(1)..(x + 2).min(self.columns);
        let y_range = y.saturating_sub(1)..(y + 2).min(self.rows);
        let mut remaining_checks = x_range
            .cartesian_product(y_range)
            .collect::<Vec<(usize, usize)>>();
        let mut ratios: Vec<u32> = Vec::new();

        while let Some(xy_check) = remaining_checks.pop() {
            if !self[xy_check].is_ascii_digit() {
                continue;
            }

            let (xc, yc) = xy_check;
            let mut xstart = xc;
            let mut xend = xc;

            while xend != self.rows && self[(xend, yc)].is_ascii_digit() {
                xend += 1;
                try_remove_coords(&mut remaining_checks, (xend, yc));
            }
            while xstart != 0 {
                if !self[(xstart - 1, yc)].is_ascii_digit() {
                    break;
                }
                xstart -= 1;
                try_remove_coords(&mut remaining_checks, (xstart, yc));
            }

            let num: u32 = self.data[(yc * self.columns + xstart)..(yc * self.columns + xend)]
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

    fn index(&self, index: (usize, usize)) -> &char {
        let (col, row) = index;
        &self.data[row * self.columns + col]
    }
}

impl IndexMut<(usize, usize)> for Schematic {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut char {
        let (col, row) = index;
        &mut self.data[row * self.columns + col]
    }
}

fn try_remove_coords(coords: &mut Vec<(usize, usize)>, xy: (usize, usize)) {
    if let Some(pos) = coords.iter().position(|&x| x == xy) {
        coords.swap_remove(pos);
    }
}
