use std::{fs::File, io::{BufReader, BufRead}};

use crate::solution::Solution;

pub fn solve_part_1() -> Solution {
	let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_02.in").unwrap();
	let reader = BufReader::new(input_file);

	reader.lines()
		.map(|line| line.unwrap())
		.map(|line| {
			let game = line.split(":").collect::<Vec<&str>>();
			let id = game[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
			let is_possible = is_game_possible(game[1]);
			(id, is_possible)
		})
		.filter(|(_, is_possible)| *is_possible)
		.map(|(id, _)| id)
		.sum::<u32>().into()
}

fn is_game_possible(game: &str) -> bool {

	const RED: u32 = 12;
	const GREEN: u32 = 13;
	const BLUE: u32 = 14;

	let mut red_count = 0;
	let mut green_count = 0;
	let mut blue_count = 0;

	for subset in game.split(";") {
		subset.split(",").for_each(|cube| {
			let tmp = cube.trim().split(" ").collect::<Vec<&str>>();
			match tmp[1] {
				"red" => red_count += tmp[0].parse::<u32>().unwrap(),
				"green" => green_count += tmp[0].parse::<u32>().unwrap(),
				"blue" => blue_count += tmp[0].parse::<u32>().unwrap(),
				_ => (),
			}
		});

		if red_count > RED || green_count > GREEN || blue_count > BLUE {
			return false;
		}

		red_count = 0;
		green_count = 0;
		blue_count = 0;
	};

	true
}

pub fn solve_part_2() -> Solution {
	let input_file = File::open("src/catalogue/advent_of_code/year_2023/inputs/day_02.in").unwrap();
	let reader = BufReader::new(input_file);

	reader.lines()
		.map(|line| line.unwrap())
		.map(|line| {
			let game = line.split(":").collect::<Vec<&str>>();
			get_min_needed_cubes(game[1])
		})
		.map(|(red_count, green_count, blue_count)| red_count * green_count * blue_count)
		.sum::<u32>().into()
}

fn get_min_needed_cubes(game: &str) -> (u32, u32, u32) {
	let mut red_count = 0;
	let mut green_count = 0;
	let mut blue_count = 0;

	for subset in game.split(";") {
		subset.split(",").for_each(|cube| {
			let tmp = cube.trim().split(" ").collect::<Vec<&str>>();
			let count = tmp[0].parse::<u32>().unwrap();
			match tmp[1] {
				"red" => if count > red_count { red_count = count },
				"green" => if count > green_count { green_count = count },
				"blue" => if count > blue_count { blue_count = count },
				_ => (),
			}
		});
	};

	(red_count, green_count, blue_count)
}