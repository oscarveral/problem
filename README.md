# Problems.

Collection of problems from different sources solved in Rust. 

The problem or problems being solved are selected via program arguments. The
program parses all arguments to obtain the corresponding solution function that
needs to be executed. Then, it executes each function and prints the problem 
name, solution obtained and execution time. It also prints the total execution 
time needed to solve all supplied problems. 

## Current sources.

As each problem is obtained by parsing program arguments, each problem source 
needs a string pattern to be able to parse problems for that source. It allows 
to run multiple problems if ranges of identifiers are specified instead of a 
single identifier.

* Advent of Code. advent_of_code:yyyy(-yyyy)/dd(-dd)/pp(-pp)

# Example of use.

* Advent of Code. Year 2021, day 2, part 1: advent_of_code:2021/02/01.
* Advent of Code. Years 2020-2023, day 2 of each years both parts of each 
	problem: advent_of_code:2020-2023/02/01-02.
* Advent of Code. The above and year 2018, day 8, both parts: 
	advent_of_code:2020-2023/02/01-02 advent_of_code:2018/08/01-02.