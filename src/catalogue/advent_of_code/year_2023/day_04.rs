use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use crate::solution::Solution;

pub fn solve_part_1() -> Solution {
    let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_04.in").unwrap();
    let reader = BufReader::new(input_file);

    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut winners: Vec<usize> = Vec::new();
            let mut numbers: Vec<usize> = Vec::new();

            let read_numbers = line
                .split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .collect::<Vec<&str>>();

            read_numbers[0].trim().split(' ').for_each(|number| {
                if !number.is_empty() {
                    winners.push(number.parse::<usize>().unwrap());
                }
            });

            read_numbers[1].trim().split(' ').for_each(|number| {
                if !number.is_empty() {
                    numbers.push(number.parse::<usize>().unwrap());
                }
            });

            (winners, numbers)
        })
        .map(|(winners, numbers)| {
            let mut count = 0;
            for w in winners {
                if numbers.contains(&w) {
                    count += 1;
                }
            }
            if count == 0 {
                0
            } else {
                2_usize.pow(count - 1)
            }
        })
        .sum::<usize>()
        .into()
}

pub fn solve_part_2() -> Solution {
    let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_04.in").unwrap();
    let reader = BufReader::new(input_file);

    let mut count_vec: HashMap<usize, usize> = HashMap::new();

    reader.lines().map(|line| line.unwrap()).for_each(|line| {
        let mut winners: Vec<usize> = Vec::new();
        let mut numbers: Vec<usize> = Vec::new();

        let mut it = line.split(':');

        let card_number = it
            .next()
            .unwrap()
            .split(' ')
            .find(|s| s.starts_with(|c: char| c.is_ascii_digit()))
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        let read_numbers = it.next().unwrap().split('|').collect::<Vec<&str>>();

        read_numbers[0].trim().split(' ').for_each(|number| {
            if !number.is_empty() {
                winners.push(number.parse::<usize>().unwrap());
            }
        });

        read_numbers[1].trim().split(' ').for_each(|number| {
            if !number.is_empty() {
                numbers.push(number.parse::<usize>().unwrap());
            }
        });

        let mut count = 0;
        for w in winners {
            if numbers.contains(&w) {
                count += 1;
            }
        }

        match count_vec.entry(card_number) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
            std::collections::hash_map::Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
        }

        for i in 1..=count {
            count_vec.entry(card_number + i).or_insert(0);
            count_vec.insert(
                card_number + i,
                count_vec[&(card_number + i)] + count_vec[&card_number],
            );
        }
    });

    count_vec.values().sum::<usize>().into()
}
