use catalogue::Problem;
use catalogue::Solver;

mod catalogue;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <problem>", args[0]);
        return;
    }

    let mut problems = Vec::new();

    for arg in &args[1..] {
        let problem = catalogue::problem_from_str(arg);
        if let Some(problem) = problem {
            problems.push(problem);
        } else {
            eprintln!("Problem {} could not be parsed.", arg);
        }
    }

    let solvers: Vec<_> = problems.iter().flat_map(|p| p.get_solver()).collect();

    let mut elapsed_total = 0;
    for solver in solvers {
        let time = std::time::Instant::now();
        let solution = solver.solve();
        let elapsed = time.elapsed().as_nanos();
        let elapsed_current = elapsed as f64 / 1_000_000.0;
        elapsed_total += elapsed;

        println!("Problem: {}", solver.get_info());
        println!("Solution: {}", solution);
        println!("Elapsed: {}ms", elapsed_current);
        println!();
    }

    println!("Total elapsed: {}ms", elapsed_total as f64 / 1_000_000.0);
}
