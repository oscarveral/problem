# Problems.

Collection of problems from different sources solved in Rust. 

The problem or problems being solved are selected via program arguments. The
program parses all arguments to obtain the corresponding solution function that
needs to be executed. Then it executes each function and prints the problem 
name, solution obtained and execution time. It also prints the total execution 
time needed to solve all problems. 

## Current sources.

As each problem is obtained by parsing program arguments, each problem source 
needs to follow a pattern to be able to parse problems for that source.

* Advent of Code (advent-of-code:yyyy-dd-pp) 