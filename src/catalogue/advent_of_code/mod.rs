mod year_2023;

use crate::router::Solver;

pub fn get_solver(year: usize, day: usize, part: usize) -> Option<Solver> {
    match year {
        2023 => year_2023::get_solver(day, part),
        _ => None,
    }
}
