use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::solution::Solution;

pub fn solve_part_1() -> Solution {
    let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_03.in").unwrap();
    let reader = BufReader::new(input_file);

    let map = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let mut y = 0;
    let mut x = 0;

    while y < map.len() {
        while x < map[y].len() {
            if map[y][x].is_ascii_digit() {
                let start = x;
                let mut end = x;

                while end < map[y].len() - 1 && map[y][end + 1].is_ascii_digit() {
                    end += 1;
                }

                let scan_start = (y.saturating_sub(1), start.saturating_sub(1));
                let scan_end = (
                    if y < map.len() - 1 { y + 1 } else { y },
                    if end < map[y].len() - 1 { end + 1 } else { end },
                );

                let mut is_valid = false;
                for (_, line) in map
                    .iter()
                    .enumerate()
                    .take(scan_end.0 + 1)
                    .skip(scan_start.0)
                {
                    for (_, scan_char) in line
                        .iter()
                        .enumerate()
                        .take(scan_end.1 + 1)
                        .skip(scan_start.1)
                    {
                        if *scan_char != '.' && !scan_char.is_ascii_digit() {
                            is_valid = true;
                        }
                    }
                }

                if is_valid {
                    sum += map[y][start..=end]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                }

                x = end;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    sum.into()
}

pub fn solve_part_2() -> Solution {
    let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_03.in").unwrap();
    let reader = BufReader::new(input_file);

    let map = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut y = 0;
    let mut x = 0;
    type Postion = (usize, usize);
    let mut gear_map: HashMap<Postion, Vec<(Postion, Postion)>> = HashMap::new();

    while y < map.len() {
        while x < map[y].len() {
            if map[y][x].is_ascii_digit() {
                let start = x;
                let mut end = x;

                while end < map[y].len() - 1 && map[y][end + 1].is_ascii_digit() {
                    end += 1;
                }

                let scan_start = (y.saturating_sub(1), start.saturating_sub(1));
                let scan_end = (
                    if y < map.len() - 1 { y + 1 } else { y },
                    if end < map[y].len() - 1 { end + 1 } else { end },
                );

                for (scan_y, line) in map
                    .iter()
                    .enumerate()
                    .take(scan_end.0 + 1)
                    .skip(scan_start.0)
                {
                    for (scan_x, scan_char) in line
                        .iter()
                        .enumerate()
                        .take(scan_end.1 + 1)
                        .skip(scan_start.1)
                    {
                        if *scan_char == '*' {
                            let gear_list = gear_map.entry((scan_y, scan_x)).or_default();
                            gear_list.push(((y, start), (y, end)));
                        }
                    }
                }

                x = end;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    gear_map
        .iter()
        .filter(|(_, gear_list)| gear_list.len() == 2)
        .map(|(_, gear_list)| {
            map[gear_list[0].0 .0][gear_list[0].0 .1..=gear_list[0].1 .1]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
                * map[gear_list[1].0 .0][gear_list[1].0 .1..=gear_list[1].1 .1]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
        })
        .sum::<usize>()
        .into()
}
