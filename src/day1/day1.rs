use crate::util;

const STARTING_POS: i16 = 50;
const NUM_POSITIONS: i16 = 100;

#[derive(PartialEq, Eq)]
enum Direction {
	Left,
	Right,
}

pub fn part1() -> usize {
	let mut pos = STARTING_POS;
	let mut sum = 0;

	for (dir, dist) in get_rotations() {
		pos = match dir {
			Direction::Left => pos - dist,
			Direction::Right => pos + dist,
		}
		.rem_euclid(NUM_POSITIONS);

		if pos == 0 {
			sum += 1;
		}
	}

	sum
}

pub fn part2() -> usize {
	let mut pos = STARTING_POS;
	let mut sum = 0;

	for (dir, dist) in get_rotations() {
		let old_pos = pos;
		let complete_turns = (dist / NUM_POSITIONS) as usize;

		match dir {
			Direction::Left => {
				pos = (pos - dist).rem_euclid(NUM_POSITIONS + 1);

				if pos >= old_pos {
					sum += 1;
				}
			}
			Direction::Right => {
				pos = (pos + dist).rem_euclid(NUM_POSITIONS);

				if pos <= old_pos {
					sum += 1;
				}
			}
		};
		sum += complete_turns;
	}

	sum
}

fn get_rotations() -> impl Iterator<Item = (Direction, i16)> {
	let regex = util::input::regex_or_panic(r"(?<dir>L|R)(?<dist>\d+)");

	util::input::read_input_lines()
		.into_iter()
		.map(move |line| {
			let capture_groups = regex
				.captures(&line.trim())
				.expect("Fatal - regex match failed");

			let direction = capture_groups["dir"].into();
			let distance = capture_groups["dist"]
				.parse()
				.expect("Fatal - failed to parse distance value as number");

			(direction, distance)
		})
}

impl From<&str> for Direction {
	fn from(value: &str) -> Self {
		match value {
			"L" => Direction::Left,
			"R" => Direction::Right,
			_ => panic!("Fatal - invalid direction"),
		}
	}
}
