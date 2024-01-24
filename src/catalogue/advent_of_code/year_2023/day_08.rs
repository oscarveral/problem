use std::str::FromStr;

super::get_input!(2023, 8);

super::create_solver!(PART_1, steps, "Advent of Code 2023, Day 8, Part 1. Haunted Wasteland.");
super::create_solver!(PART_2, steps_simultaneous, "Advent of Code 2023, Day 8, Part 2. Haunted Wasteland.");

struct ParseError;

#[derive(Clone, Copy, Debug)]
struct Node {
	id: u16,
	children: (u16, u16),
}

impl Node {
	fn get_id(&self) -> u16 {
		self.id
	}

	fn ends_with_a(&self) -> bool {
		self.id % 26 == 0
	}

	fn ends_with_z(&self) -> bool {
		self.id % 26 == 25
	}

	fn is_zzz(&self) -> bool {
		self.id == 25 * 26 * 26 + 25 * 26 + 25
	}
}

#[derive(Clone, Copy, Debug)]
enum Direction {
	Left,
	Right,
}

struct Map {
	instructions: Vec<Direction>,
	starts: Vec<Node>,
	nodes: Vec<Node>,
}

impl FromStr for Node{
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.split_once(" = ").ok_or(ParseError)?;

		let id = parts.0.get(0..=2).ok_or(ParseError)?;
		let id = calculate_id(id)?;

		let children = parts.1
			.trim_matches(|c| c == '(' || c == ')')
			.split_once(", ")
			.ok_or(ParseError)?;
		let children = (calculate_id(children.0)?, calculate_id(children.1)?);

		Ok(Node { id, children })
	}
}

impl FromStr for Map {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {

		let (dirs, nodes) = s.split_once("\n\n").ok_or(ParseError)?;

		let instructions = dirs.chars().filter_map(|c| match c {
			'L' => Some(Direction::Left),
			'R' => Some(Direction::Right),
			_ => None,
		}).collect();


		let mut node_list = vec![Node { id: 0, children: (0, 0) }; 26 * 26 * 26];
		let mut node_starts = vec![];
		nodes.lines()
			.map(|l| l.parse().unwrap_or_else(|_| panic!("Failed to parse node.")))
			.for_each(|n: Node| {
				let index = n.get_id() as usize;
				node_list[index] = n;
				if n.ends_with_a() {
					node_starts.push(n);
				}
			});

		Ok(Map { instructions, nodes: node_list, starts: node_starts})
	}
}

impl Map {
	fn next(&self, node: &Node, dir: Direction) -> &Node {
		match dir {
			Direction::Left => &self.nodes[node.children.0 as usize],
			Direction::Right => &self.nodes[node.children.1 as usize],
		}
	}

	fn steps_to_dest(&self, start: &Node, is_dest: impl Fn(&Node) -> bool) -> u64 {
		let mut steps = 0;
		let mut current = start;

		for i in self.instructions.iter().cycle() {

			let next = self.next(current, *i);

			steps += 1;
			current = next;

			if is_dest(current) {
				break;
			}
		}
		steps
	}

	fn steps_simultaneous<T: Fn(&Node) -> bool>(&self, is_dest: T) -> u64 {
		
		self.starts.iter()
			.map(|n| self.steps_to_dest(n, &is_dest))
			.reduce(lcm)
			.unwrap_or_else(|| panic!("Failed to calculate steps."))
	}

	
}

fn calculate_id(s: &str) -> Result<u16, ParseError> {
	if s.len() != 3 {
		return Err(ParseError);
	}

	s.chars()
		.map(|c| c as u16 -'A' as u16)
		.reduce(|acc, b| acc * 26 + b)
		.ok_or(ParseError)
}

fn steps() -> u64 {
	let input = get_input();

	let map = input.parse::<Map>()
		.unwrap_or_else(|_| panic!("Failed to parse input"));

	let start = &map.nodes[calculate_id("AAA").unwrap_or_else(|_| panic!("Failed to calculate id")) as usize];

	map.steps_to_dest(start, Node::is_zzz)

}

fn steps_simultaneous() -> u64 {
	let input = get_input();

	let map = input.parse::<Map>()
		.unwrap_or_else(|_| panic!("Failed to parse input"));

	map.steps_simultaneous(Node::ends_with_z)
}

fn gcd(a: u64, b: u64) -> u64 {
	let mut a = a;
	let mut b = b;
	while a != b {
		if a > b {
			a -= b;
		} else {
			b -= a;
		}
	}
	a
}

fn lcm(a: u64, b: u64) -> u64 {
	a * b / gcd(a, b)
}