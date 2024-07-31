use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Coord {
	pub column: usize,
	pub row: usize,
}

impl Coord {
	pub fn parse(input: &str) -> Coord {
		let input = input.to_uppercase();
		let mut row_string = String::new();

		let mut column = 0;
		input.chars().for_each(|item| {
			if item.is_alphabetic() {
				let value = (item as usize) - ('A' as usize) + 1;
				column = (26 * column) + value;
			} else if item.is_numeric() {
				row_string.push(item);
			}
		});
		column -= 1;

		let row = row_string.parse::<usize>().unwrap_or(1) - 1;

		Coord { column, row }
	}

	pub fn stringify(&self) -> String {
		let mut result = Vec::new();
		let mut column = self.column + 1;

		while column > 0 {
			column -= 1;
			let remainder = (column % 26) as u8;
			let letter = (b'A' + remainder) as char;
			result.push(letter);
			column /= 26;
		}

		result.reverse();

		result.iter().collect::<String>() + &(self.row + 1).to_string()
	}

	pub fn is_coord(item: &str) -> bool {
		let mut has_letters = false;
		let mut has_numbers = false;
		let mut is_coord = true;

		for thing in item.chars() {
			if thing.is_ascii_alphabetic() {
				has_letters = true;
				if has_numbers {
					is_coord = false;
					break;
				}
			} else if thing.is_ascii_digit() {
				has_numbers = true;
				if !has_letters {
					is_coord = false;
					break;
				}
			} else {
				is_coord = false;
				break;
			}
		}

		if has_letters ^ has_numbers {
			false
		} else {
			is_coord
		}
	}
}

#[test]
fn parse_test() {
	assert_eq!(Coord::parse("A1"), Coord { column: 0, row: 0 });
	assert_eq!(Coord::parse("a1"), Coord { column: 0, row: 0 });
	assert_eq!(Coord::parse("a10"), Coord { column: 0, row: 9 });
	assert_eq!(Coord::parse("a510"), Coord { column: 0, row: 509 });
	assert_eq!(Coord::parse("a-1"), Coord { column: 0, row: 0 });
	assert_eq!(Coord::parse("b 5#"), Coord { column: 1, row: 4 });
	assert_eq!(Coord::parse("B2"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::parse("Z2"), Coord { column: 25, row: 1 });
	assert_eq!(Coord::parse("AA2"), Coord { column: 26, row: 1 });
	assert_eq!(Coord::parse("AB2"), Coord { column: 27, row: 1 });
	assert_eq!(Coord::parse("AZ2"), Coord { column: 51, row: 1 });
	assert_eq!(Coord::parse("BA2"), Coord { column: 52, row: 1 });
	assert_eq!(Coord::parse("BZ2"), Coord { column: 77, row: 1 });
	assert_eq!(Coord::parse("CA2"), Coord { column: 78, row: 1 });
	assert_eq!(Coord::parse("ZA2"), Coord { column: 676, row: 1 });
	assert_eq!(Coord::parse("ZZ2"), Coord { column: 701, row: 1 });
	assert_eq!(Coord::parse("AAA2"), Coord { column: 702, row: 1 });
	assert_eq!(Coord::parse("AAB2"), Coord { column: 703, row: 1 });
	assert_eq!(
		Coord::parse("ZZZ50"),
		Coord {
			column: 18_277,
			row: 49
		}
	);
	assert_eq!(
		Coord::parse("FZZ123412021312"),
		Coord {
			column: 4_757,
			row: 123_412_021_311
		}
	);
}

#[test]
fn stringify_test() {
	assert_eq!(Coord { column: 0, row: 0 }.stringify(), String::from("A1"));
	assert_eq!(Coord { column: 1, row: 1 }.stringify(), String::from("B2"));
	assert_eq!(Coord { column: 25, row: 1 }.stringify(), String::from("Z2"));
	assert_eq!(Coord { column: 26, row: 4 }.stringify(), String::from("AA5"));
	assert_eq!(Coord { column: 701, row: 4 }.stringify(), String::from("ZZ5"));
	assert_eq!(Coord { column: 702, row: 4 }.stringify(), String::from("AAA5"));
	assert_eq!(
		Coord {
			column: 20_000,
			row: 1000
		}
		.stringify(),
		String::from("ACOG1001")
	);
}

#[test]
fn is_coord_test() {
	assert_eq!(Coord::is_coord("A1"), true);
	assert_eq!(Coord::is_coord("AA11"), true);
	assert_eq!(Coord::is_coord("ZZZZ1"), true);
	assert_eq!(Coord::is_coord("Z1234"), true);
	assert_eq!(Coord::is_coord("a1"), true);

	assert_eq!(Coord::is_coord("1"), false);
	assert_eq!(Coord::is_coord("A"), false);
	assert_eq!(Coord::is_coord("A1A"), false);
	assert_eq!(Coord::is_coord("1A1"), false);
	assert_eq!(Coord::is_coord("A1A1"), false);
	assert_eq!(Coord::is_coord("AAA123.123"), false);
	assert_eq!(Coord::is_coord("A 1"), false);
	assert_eq!(Coord::is_coord("AB2/3"), false);
}

impl fmt::Display for Coord {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.stringify())
	}
}

#[test]
fn to_string_test() {
	assert_eq!(format!("{}", Coord { column: 0, row: 0 }), String::from("A1"));
	assert_eq!(format!("{}", Coord { column: 1, row: 1 }), String::from("B2"));
	assert_eq!(format!("{}", Coord { column: 25, row: 1 }), String::from("Z2"));
	assert_eq!(format!("{}", Coord { column: 26, row: 4 }), String::from("AA5"));
	assert_eq!(format!("{}", Coord { column: 701, row: 4 }), String::from("ZZ5"));
	assert_eq!(format!("{}", Coord { column: 702, row: 4 }), String::from("AAA5"));
	assert_eq!(
		format!(
			"{}",
			Coord {
				column: 20_000,
				row: 1000
			}
		),
		String::from("ACOG1001")
	);
}
