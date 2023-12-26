mod catalogue;
mod problem;
mod router;
mod solution;

use problem::Problem;
use router::Solver;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprint!("No arguments provided.");
        return;
    }

    args.swap_remove(0);

    let problems: Vec<Problem> = problem::get_problems(args);

    if problems.is_empty() {
        eprint!("No problems to solve.");
        return;
    }

    let solvers: Vec<(Solver, Problem)> = router::get_solvers(problems);

    if solvers.is_empty() {
        eprint!("No solvers to run.");
        return;
    }

    for solver in solvers {
        let time = std::time::Instant::now();
        let solution = solver.0();
        let elapsed = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        println!("Problem: {}", solver.1);
        println!("Solution: {}", solution);
        println!("Elapsed: {}ms", elapsed);
    }
}
