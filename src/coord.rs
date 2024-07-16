use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Coord {
	column: u64,
	row: u64,
}

impl Coord {
	fn from_string(input: &str) -> Coord {
		let input = input.to_uppercase();
		let mut column_string = String::new();
		let mut row_string = String::new();

		for item in input.chars() {
			if item.is_alphabetic() {
				column_string.push(item);
			} else if item.is_numeric() {
				row_string.push(item);
			}
		}

		let mut column = 0;
		for letter in column_string.chars() {
			let value = (letter as u64) - ('A' as u64) + 1;
			column = (26 * column) + value;
		}

		let row = row_string.parse::<u64>().unwrap_or(1);

		Coord { column, row }
	}
}

#[test]
fn from_string_test() {
	assert_eq!(Coord::from_string("A1"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::from_string("a1"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::from_string("a10"), Coord { column: 1, row: 10 });
	assert_eq!(Coord::from_string("a510"), Coord { column: 1, row: 510 });
	assert_eq!(Coord::from_string("a-1"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::from_string("a 5#"), Coord { column: 1, row: 5 });
	assert_eq!(Coord::from_string("B2"), Coord { column: 2, row: 2 });
	assert_eq!(Coord::from_string("Z2"), Coord { column: 26, row: 2 });
	assert_eq!(Coord::from_string("AA2"), Coord { column: 27, row: 2 });
	assert_eq!(Coord::from_string("AB2"), Coord { column: 28, row: 2 });
	assert_eq!(Coord::from_string("AZ2"), Coord { column: 52, row: 2 });
	assert_eq!(Coord::from_string("BA2"), Coord { column: 53, row: 2 });
	assert_eq!(Coord::from_string("BZ2"), Coord { column: 78, row: 2 });
	assert_eq!(Coord::from_string("CA2"), Coord { column: 79, row: 2 });
	assert_eq!(Coord::from_string("ZA2"), Coord { column: 677, row: 2 });
	assert_eq!(Coord::from_string("ZZ2"), Coord { column: 702, row: 2 });
	assert_eq!(Coord::from_string("AAA2"), Coord { column: 703, row: 2 });
	assert_eq!(Coord::from_string("AAB2"), Coord { column: 704, row: 2 });
	assert_eq!(
		Coord::from_string("ZZZ50"),
		Coord {
			column: 18_278,
			row: 50
		}
	);
}

impl fmt::Display for Coord {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut result = Vec::new();
		let mut column = self.column;

		while column > 0 {
			column -= 1; // Adjust for 1-based indexing
			let remainder = (column % 26) as u8;
			let letter = (b'A' + remainder) as char;
			result.push(letter);
			column /= 26;
		}

		result.reverse();

		write!(f, "{}{}", result.into_iter().collect::<String>(), self.row)
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
