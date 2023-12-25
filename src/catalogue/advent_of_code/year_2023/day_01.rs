use crate::solution::Solution;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn solve_part_1() -> Solution {
	let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_01.in").unwrap();
	let reader = BufReader::new(input_file);

	reader.lines()
		.map(|line| line.unwrap())
		.map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<char>>())
		.map(|line| {
			let first = line.first().unwrap();
			let last = line.last().unwrap();
			format!("{}{}", first, last)
		})
		.map(|line| line.parse::<u32>().unwrap())
		.sum::<u32>().into()
}

pub fn solve_part_2() -> Solution {
	let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_01.in").unwrap();
	let reader = BufReader::new(input_file);

	reader.lines()
		.map(|line| line.unwrap())
		.map(|line| line.replace("one", "one1one").replace("two", "two2two").replace("three", "three3three").replace("four", "four4four").replace("five", "five5five").replace("six", "six6six").replace("seven", "seven7seven").replace("eight", "eight8eight").replace("nine", "nine9nine"))
		.map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<char>>())
		.map(|line| {
			let first = line.first().unwrap();
			let last = line.last().unwrap();
			format!("{}{}", first, last)
		})
		.map(|line| line.parse::<u32>().unwrap())
		.sum::<u32>().into()
}