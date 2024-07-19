use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Coord {
	pub column: usize,
	pub row: usize,
}

impl Coord {
	pub fn parse(input: &str) -> Coord {
		let input = input.to_uppercase();
		let mut row_string = String::new();

		let mut column = 0;
		for item in input.chars() {
			if item.is_alphabetic() {
				let value = (item as usize) - ('A' as usize) + 1;
				column = (26 * column) + value;
			} else if item.is_numeric() {
				row_string.push(item);
			}
		}

		let row = row_string.parse::<usize>().unwrap_or(1);

		Coord { column, row }
	}

	pub fn stringify(&self) -> String {
		let mut result = Vec::new();
		let mut column = self.column;

		while column > 0 {
			column -= 1;
			let remainder = (column % 26) as u8;
			let letter = (b'A' + remainder) as char;
			result.push(letter);
			column /= 26;
		}

		result.reverse();

		format!("{}{}", result.iter().collect::<String>(), self.row)
	}

	pub fn is_coord(item: &str) -> bool {
		let mut has_letters = false;
		let mut has_numbers = false;
		let mut is_coord = true;

		item.chars().for_each(|thing| {
			if thing.is_alphabetic() {
				has_letters = true;
				if has_numbers {
					is_coord = false;
				}
			}

			if thing.is_numeric() {
				has_numbers = true;
				if !has_letters {
					is_coord = false;
				}
			}
		});

		if !has_letters || !has_numbers {
			false
		} else {
			is_coord
		}
	}
}

#[test]
fn parse_test() {
	assert_eq!(Coord::parse("A1"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::parse("a1"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::parse("a10"), Coord { column: 1, row: 10 });
	assert_eq!(Coord::parse("a510"), Coord { column: 1, row: 510 });
	assert_eq!(Coord::parse("a-1"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::parse("b 5#"), Coord { column: 2, row: 5 });
	assert_eq!(Coord::parse("B2"), Coord { column: 2, row: 2 });
	assert_eq!(Coord::parse("Z2"), Coord { column: 26, row: 2 });
	assert_eq!(Coord::parse("AA2"), Coord { column: 27, row: 2 });
	assert_eq!(Coord::parse("AB2"), Coord { column: 28, row: 2 });
	assert_eq!(Coord::parse("AZ2"), Coord { column: 52, row: 2 });
	assert_eq!(Coord::parse("BA2"), Coord { column: 53, row: 2 });
	assert_eq!(Coord::parse("BZ2"), Coord { column: 78, row: 2 });
	assert_eq!(Coord::parse("CA2"), Coord { column: 79, row: 2 });
	assert_eq!(Coord::parse("ZA2"), Coord { column: 677, row: 2 });
	assert_eq!(Coord::parse("ZZ2"), Coord { column: 702, row: 2 });
	assert_eq!(Coord::parse("AAA2"), Coord { column: 703, row: 2 });
	assert_eq!(Coord::parse("AAB2"), Coord { column: 704, row: 2 });
	assert_eq!(
		Coord::parse("ZZZ50"),
		Coord {
			column: 18_278,
			row: 50
		}
	);
}

#[test]
fn stringify_test() {
	assert_eq!(Coord { column: 1, row: 1 }.stringify(), String::from("A1"));
	assert_eq!(Coord { column: 2, row: 2 }.stringify(), String::from("B2"));
	assert_eq!(Coord { column: 26, row: 2 }.stringify(), String::from("Z2"));
	assert_eq!(Coord { column: 27, row: 5 }.stringify(), String::from("AA5"));
	assert_eq!(Coord { column: 702, row: 5 }.stringify(), String::from("ZZ5"));
	assert_eq!(Coord { column: 703, row: 5 }.stringify(), String::from("AAA5"));
	assert_eq!(
		Coord {
			column: 20_000,
			row: 1000
		}
		.stringify(),
		String::from("ACOF1000")
	);
}

#[test]
fn is_coord_test() {
	assert_eq!(Coord::is_coord(&String::from("A1")), true);
	assert_eq!(Coord::is_coord(&String::from("AA11")), true);
	assert_eq!(Coord::is_coord(&String::from("ZZZZ1")), true);
	assert_eq!(Coord::is_coord(&String::from("Z1234")), true);

	assert_eq!(Coord::is_coord(&String::from("1")), false);
	assert_eq!(Coord::is_coord(&String::from("A")), false);
	assert_eq!(Coord::is_coord(&String::from("A1A")), false);
	assert_eq!(Coord::is_coord(&String::from("1A1")), false);
}

impl fmt::Display for Coord {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.stringify())
	}
}

#[test]
fn to_string_test() {
	assert_eq!(Coord { column: 1, row: 1 }.to_string(), String::from("A1"));
	assert_eq!(Coord { column: 2, row: 2 }.to_string(), String::from("B2"));
	assert_eq!(Coord { column: 26, row: 2 }.to_string(), String::from("Z2"));
	assert_eq!(Coord { column: 27, row: 5 }.to_string(), String::from("AA5"));
	assert_eq!(Coord { column: 702, row: 5 }.to_string(), String::from("ZZ5"));
	assert_eq!(Coord { column: 703, row: 5 }.to_string(), String::from("AAA5"));
	assert_eq!(
		Coord {
			column: 20_000,
			row: 1000
		}
		.to_string(),
		String::from("ACOF1000")
	);
}
