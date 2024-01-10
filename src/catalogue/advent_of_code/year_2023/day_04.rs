use std::fs;

use crate::solution::Solution;

fn get_input() -> String {
    fs::read_to_string("src/catalogue/advent_of_code/year_2023/inputs/day_04.in").unwrap()
}

fn winning_num_count(card: &str) -> Option<u32> {
    let (_, numbers) = card.split_once(':')?;
    let (winners, numbers) = numbers.split_once('|')?;
    let mut winning = [false; 100];
    for num in winners.split_whitespace().map(|n| n.parse::<usize>()) {
        winning[num.ok()?] = true;
    }
    Some(
        numbers
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .filter(|&n| winning[n])
            .count() as u32,
    )
}

pub fn solve_part_1() -> Solution {
    let input = get_input();

    input
        .lines()
        .filter_map(winning_num_count)
        .filter(|&count| count > 0)
        .map(|c| 1 << (c - 1))
        .sum::<u32>()
        .into()
}

pub fn solve_part_2() -> Solution {
    let input = get_input();

    let lines = input.lines().collect::<Vec<_>>();
    let mut cards = vec![1; lines.len()];
    let mut total = 0;
    for (i, line) in lines.into_iter().enumerate() {
        let card_copies = cards[i];
        total += card_copies;
        let winners = winning_num_count(line).unwrap_or(0) as usize;
        for j in 0..winners {
            if let Some(card) = cards.get_mut(i + j + 1) {
                *card += card_copies;
            }
        }
    }
    total.into()
}
