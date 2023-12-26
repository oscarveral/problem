mod advent_of_code;

use crate::problem::Problem;
use crate::router::Solver;

pub fn get_solver(problem: &Problem) -> Option<Solver> {
    match problem {
        Problem::AdventOfCode(year, day, part) => advent_of_code::get_solver(*year, *day, *part),
        _ => None,
    }
}
