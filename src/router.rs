use crate::problem::Problem;
use crate::solution::Solution;

use crate::catalogue::get_solver;

pub type Solver  = fn() -> Solution;

pub fn get_solvers(problems: Vec<Problem>) -> Vec<(Solver, Problem)> {
	let mut solvers: Vec<(Solver, Problem)> = Vec::new();
	for problem in problems {
		let solver = get_solver(&problem);
		if solver.is_none() {
			eprintln!("Can't find the solver for: {}", problem);
			continue;
		}
		solvers.push((solver.unwrap(), problem));
	}
	solvers
}
