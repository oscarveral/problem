mod day_01;
mod day_02;
mod day_03;

use crate::router::Solver;

pub fn get_solver(day: usize, part: usize) -> Option<Solver> {
	match (day, part) {
		(1, 1) => Some(day_01::solve_part_1),
		(1, 2) => Some(day_01::solve_part_2),
		(2, 1) => Some(day_02::solve_part_1),
		(2, 2) => Some(day_02::solve_part_2),
		_ => None,
	}
}