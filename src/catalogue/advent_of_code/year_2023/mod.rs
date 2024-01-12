mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

use crate::router::Solver;

pub fn get_solver(day: usize, part: usize) -> Option<Solver> {
    match (day, part) {
        (1, 1) => Some(day_01::solve_part_1),
        (1, 2) => Some(day_01::solve_part_2),
        (2, 1) => Some(day_02::solve_part_1),
        (2, 2) => Some(day_02::solve_part_2),
        (3, 1) => Some(day_03::solve_part_1),
        (3, 2) => Some(day_03::solve_part_2),
        (4, 1) => Some(day_04::solve_part_1),
        (4, 2) => Some(day_04::solve_part_2),
        (5, 1) => Some(day_05::solve_part_1),
        (5, 2) => Some(day_05::solve_part_2),
        (6, 1) => Some(day_06::solve_part_1),
        (6, 2) => Some(day_06::solve_part_2),
        _ => None,
    }
}
