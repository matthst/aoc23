use std::fs::read_to_string;

mod util;

fn main() {
    let input: String = read_to_string("res/day02").unwrap();
    util::time_function("Part 1", input.clone(), part1);
    util::time_function("Part 2", input.clone(), part2);
}

fn part1(input: String) -> String {
    let sum: i32 = input
        .lines()
        .map(|s| Game::new(s))
        .filter(|g| {
            g.parts
                .iter()
                .all(|p| p[0] <= 12 && p[1] <= 13 && p[2] <= 14)
        })
        .map(|g| g.id)
        .sum();

    sum.to_string()
}

fn part2(input: String) -> String {
    let sum: i32 = input
        .lines()
        .map(|s| Game::new(s))
        .map(|g| get_max_cubes(&g.parts).iter().product::<i32>())
        .sum();

    sum.to_string()
}

fn get_max_cubes(v: &Vec<[i32; 3]>) -> [i32; 3] {
    v.iter().fold([0; 3], |max, arr| {
        [max[0].max(arr[0]), max[1].max(arr[1]), max[2].max(arr[2])]
    })
}

struct Game {
    id: i32,
    parts: Vec<[i32; 3]>,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let sp: Vec<&str> = input.split(':').collect::<Vec<&str>>();
        let id = sp[0][5..].parse::<i32>().unwrap();

        let parts = sp[1]
            .split(';')
            .map(|part| {
                let mut arr = [0; 3];
                for color in part
                    .split(',')
                    .map(|s| s.trim().split(' ').collect::<Vec<&str>>())
                {
                    let count = color[0].parse::<i32>().unwrap();
                    match color[1] {
                        "red" => arr[0] = count,
                        "green" => arr[1] = count,
                        "blue" => arr[2] = count,
                        _ => panic!(),
                    }
                }
                arr
            })
            .collect::<Vec<[i32; 3]>>();

        Game { id, parts }
    }
}
