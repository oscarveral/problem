use std::{char, collections::HashMap, fs};

use crate::solution::Solution;

fn get_input() -> String {
    fs::read_to_string("src/catalogue/advent_of_code/year_2023/inputs/day_03.in").unwrap()
}

pub fn solve_part_1() -> Solution {
    let input = get_input();
    let mut part_sum = 0;
    let mut cur_num: Option<u32> = None;
    let mut cur_start: Option<usize> = None;
    let chars: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    for (row, line) in chars.iter().enumerate() {
        for (col, ch) in line.iter().copied().enumerate() {
            let mut numeric = false;
            if let Some(digit) = ch.to_digit(10) {
                numeric = true;
                cur_num = Some(cur_num.unwrap_or(0) * 10 + digit);
                if cur_start.is_none() {
                    cur_start = Some(col);
                }
            }
            if !numeric || col == line.len() - 1 {
                if let Some((start, number)) = cur_start.zip(cur_num) {
                    let end = if numeric { col } else { col - 1 };
                    let mut symbol: Option<char> = None;
                    for i in row.saturating_sub(1)..=row.saturating_add(1) {
                        for j in start.saturating_sub(1)..=end.saturating_add(1) {
                            if let Some(&adj_ch) = chars.get(i).and_then(|l| l.get(j)) {
                                if !adj_ch.is_numeric() && adj_ch != '.' {
                                    symbol = Some(adj_ch);
                                }
                            }
                        }
                    }
                    if symbol.is_some() {
                        part_sum += number;
                    }
                }
                cur_start = None;
                cur_num = None;
            }
        }
    }
    part_sum.into()
}

pub fn solve_part_2() -> Solution {
    let input = get_input();
    let mut gears: HashMap<(usize, usize), Gear> = HashMap::new();
    let mut cur_num: Option<u32> = None;
    let mut cur_start: Option<usize> = None;
    let chars: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    // TODO: scan for gears instead, and only parse numbers when gears are found?
    for (row, line) in chars.iter().enumerate() {
        for (col, ch) in line.iter().copied().enumerate() {
            let mut numeric = false;
            if let Some(digit) = ch.to_digit(10) {
                numeric = true;
                cur_num = Some(cur_num.unwrap_or(0) * 10 + digit);
                if cur_start.is_none() {
                    cur_start = Some(col);
                }
            }
            if !numeric || col == line.len() - 1 {
                if let Some((start, number)) = cur_start.zip(cur_num) {
                    let end = if numeric { col } else { col - 1 };
                    let mut counted_gear_part = false;
                    for i in row.saturating_sub(1)..=row.saturating_add(1) {
                        for j in start.saturating_sub(1)..=end.saturating_add(1) {
                            if let Some(&adj_ch) = chars.get(i).and_then(|l| l.get(j)) {
                                if adj_ch == '*' && !counted_gear_part {
                                    let gear = gears.entry((i, j)).or_insert(Gear {
                                        ratio: 1,
                                        adjacent_parts: 0,
                                    });
                                    gear.ratio *= number;
                                    gear.adjacent_parts += 1;
                                    counted_gear_part = true;
                                }
                            }
                        }
                    }
                }
                cur_start = None;
                cur_num = None;
            }
        }
    }

    gears
        .into_values()
        .filter_map(|gear| (gear.adjacent_parts == 2).then_some(gear.ratio))
        .sum::<u32>()
        .into()
}

struct Gear {
    ratio: u32,
    adjacent_parts: usize,
}
