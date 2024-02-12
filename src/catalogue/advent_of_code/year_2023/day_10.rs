use std::str::FromStr;

super::get_input!(2023, 10);

super::create_solver!(PART_1, input_loop_length, "Advent of Code 2023, Day 10, Part 1. Pipe Maze.");

super::create_solver!(PART_2, calculate_inside_count, "Advent of Code 2023, Day 10, Part 2. Pipe Maze.");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
	row: usize,
	col: usize,
}

impl Position {
	fn go(&self, dir: &Direction) -> Option<Self> {
		Some(match dir {
			Direction::Down => Self {
				row: self.row.checked_add(1)?,
				col: self.col,
			},
			Direction::Up => Self {
				row: self.row.checked_sub(1)?,
				col: self.col,
			},
			Direction::Left => Self {
				row: self.row,
				col: self.col.checked_sub(1)?,
			},
			Direction::Right => Self {
				row: self.row,
				col: self.col.checked_add(1)?,
			},
		})
	} 
}

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Pipe {
	Vertical,
	Horizontal,
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
	None,
	Start,
}

impl Pipe {
	fn next_direction(&self, dir: Direction) -> Option<Direction>{
		match (self, dir) {
			(Pipe::Vertical, Direction::Up) => Some(Direction::Up),
			(Pipe::Vertical, Direction::Down) => Some(Direction::Down),
			(Pipe::Horizontal, Direction::Left) => Some(Direction::Left),
			(Pipe::Horizontal, Direction::Right) => Some(Direction::Right),
			(Pipe::TopLeft, Direction::Down) => Some(Direction::Left),
			(Pipe::TopLeft, Direction::Right) => Some(Direction::Up),
			(Pipe::TopRight, Direction::Down) => Some(Direction::Right),
			(Pipe::TopRight, Direction::Left) => Some(Direction::Up),
			(Pipe::BottomLeft, Direction::Up) => Some(Direction::Left),
			(Pipe::BottomLeft, Direction::Right) => Some(Direction::Down),
			(Pipe::BottomRight, Direction::Up) => Some(Direction::Right),
			(Pipe::BottomRight, Direction::Left) => Some(Direction::Down),
			_ => None,
		}
	}

	fn start_direction(&self) -> Option<Direction> {
		match self {
			Pipe::Vertical => Some(Direction::Down),
			Pipe::Horizontal => Some(Direction::Right),
			Pipe::TopLeft => Some(Direction::Down),
			Pipe::TopRight => Some(Direction::Down),
			Pipe::BottomLeft => Some(Direction::Right),
			Pipe::BottomRight => Some(Direction::Right),
			_ => None,
		}
	}
}

struct Map {
	map: Vec<Pipe>,
	rows: usize,
	cols: usize,
	start: Position,
}

impl From<char> for Pipe {
	fn from(value: char) -> Self {
		match value {
			'|' => Self::Vertical,
			'-' => Self::Horizontal,
			'L' => Self::TopRight,
			'J' => Self::TopLeft,
			'7' => Self::BottomLeft,
			'F' => Self::BottomRight,
			'S' => Self::Start,
			_ => Self::None,
		}
	}
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Map {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut rows = 0;
		let mut cols = 0;
		let mut start = Position { row: 0, col: 0 };
		let mut map = Vec::new();

		for (row, line) in s.lines().enumerate() {
			rows += 1;
			cols = 0;
			for (col, c) in line.chars().enumerate() {
				cols += 1;
				let pipe = Pipe::from(c);
				if pipe == Pipe::Start {
					start = Position {
						row: row,
						col: col,
					};
				}
				map.push(pipe);
			}
		}

		let mut map = Self {
			map,
			rows,
			cols: cols,
			start,
		};

		let top = map.get(map.start.go(&Direction::Up).unwrap_or(map.start));
		let bottom = map.get(map.start.go(&Direction::Down).unwrap_or(map.start));
		let left = map.get(map.start.go(&Direction::Left).unwrap_or(map.start));
		let right = map.get(map.start.go(&Direction::Right).unwrap_or(map.start));

		let tops_points = match top {
			Some(Pipe::Vertical) => true,
			Some(Pipe::BottomLeft) => true,
			Some(Pipe::BottomRight) => true,
			_ => false,
			
		};

		let bottom_points = match bottom {
			Some(Pipe::Vertical) => true,
			Some(Pipe::TopLeft) => true,
			Some(Pipe::TopRight) => true,
			_ => false,
		};

		let left_points = match left {
			Some(Pipe::Horizontal) => true,
			Some(Pipe::TopRight) => true,
			Some(Pipe::BottomRight) => true,
			_ => false,
		};

		let right_points = match right {
			Some(Pipe::Horizontal) => true,
			Some(Pipe::TopLeft) => true,
			Some(Pipe::BottomLeft) => true,
			_ => false,
		};

		let start_pipe = match (tops_points, bottom_points, left_points, right_points) {
			(true, true, false, false) => Pipe::Vertical,
			(false, false, true, true) => Pipe::Horizontal,
			(true, false, false, true) => Pipe::TopRight,
			(true, false, true, false) => Pipe::TopLeft,
			(false, true, false, true) => Pipe::BottomRight,
			(false, true, true, false) => Pipe::BottomLeft,
			_ => return Err(ParseError),
		};

		map.set(map.start, start_pipe);

		Ok(map)
	}
}

impl Map {
	fn index(&self, pos: Position) -> usize{
		pos.row * self.cols + pos.col
	}

	fn get(&self, pos: Position) -> Option<&Pipe> {
		self.map.get(self.index(pos))
	}

	fn set(&mut self, pos: Position, pipe: Pipe) {
		let index = self.index(pos);
		self.map[index] = pipe;
	}

	fn calcuate_loop_length(&self) -> usize {
		let mut current_pos = self.start;
		let mut current_pipe = self.get(current_pos).unwrap();
		let mut current_dir = current_pipe.start_direction().unwrap();
		let mut steps = 1;

		loop {
			current_pos = current_pos.go(&current_dir).unwrap();
			
			if current_pos == self.start {
				return steps / 2;
			}

			current_pipe = self.get(current_pos).unwrap();
			current_dir = current_pipe.next_direction(current_dir).unwrap();
			steps += 1;
		}
	}

	fn clear_junk(&mut self) {
		let mut current_pos = self.start;
		let mut current_pipe = self.get(current_pos).unwrap();
		let mut current_dir = current_pipe.start_direction().unwrap();

		let mut new_pipes = vec![Pipe::None; self.map.len()];
		new_pipes[self.index(current_pos)] = *current_pipe;

		loop {
			current_pos = current_pos.go(&current_dir).unwrap();
			
			if current_pos == self.start {
				break;
			}

			current_pipe = self.get(current_pos).unwrap();
			new_pipes[self.index(current_pos)] = *current_pipe;
			current_dir = current_pipe.next_direction(current_dir).unwrap();
		}

		self.map = new_pipes;
	}
}

fn input_loop_length() -> usize {
	let map = get_input().parse::<Map>().unwrap();
	map.calcuate_loop_length()
}

fn calculate_inside_count() -> usize {
	let mut map = get_input().parse::<Map>().unwrap();
	map.clear_junk();

	let mut count = 0;

	for i in 0..map.rows {
		let mut inside = false;
		for j in 0..map.cols {
			match *map.get(Position { row: i, col: j }).unwrap() {
				Pipe::Vertical | Pipe::BottomLeft | Pipe::BottomRight => inside = !inside,
				Pipe::None if inside => count += 1,
				_ => (),
			}
		}
	}

	count
}