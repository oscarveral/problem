mod day_01;

use crate::router::Solver;

pub fn get_solver(day: usize, part: usize) -> Option<Solver> {
	match (day, part) {
		(1, 1) => Some(day_01::solve_part_1),
		(1, 2) => Some(day_01::solve_part_2),
		_ => None,
	}
}