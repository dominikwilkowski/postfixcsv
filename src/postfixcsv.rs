use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Coord {
	column: u64,
	row: u64,
}

impl Coord {
	fn from_string(input: &str) -> Coord {
		let mut column_string = String::new();
		let mut row_string = String::new();
		let mut base = 1;

		for item in input.chars() {
			if item.is_alphabetic() {
				column_string.push(item);
			} else {
				row_string.push(item);
			}
		}

		let mut column = 0;
		for letter in column_string.chars() {
			let value = (letter as u64 - 'A' as u64 + 1) * base;
			column += value;
			base *= 26;
		}

		let row = row_string.parse::<u64>().unwrap_or(0);

		Coord { column, row }
	}
}

#[test]
fn from_string_test() {
	assert_eq!(Coord::from_string("A1"), Coord { column: 1, row: 1 });
	assert_eq!(Coord::from_string("B2"), Coord { column: 2, row: 2 });
	assert_eq!(Coord::from_string("Z2"), Coord { column: 26, row: 2 });
	assert_eq!(Coord::from_string("AA2"), Coord { column: 27, row: 2 });
	assert_eq!(Coord::from_string("AB2"), Coord { column: 28, row: 2 });
	assert_eq!(Coord::from_string("AZ2"), Coord { column: 52, row: 2 });
	assert_eq!(Coord::from_string("BA2"), Coord { column: 53, row: 2 });
	assert_eq!(Coord::from_string("BZ2"), Coord { column: 78, row: 2 });
	assert_eq!(Coord::from_string("CA2"), Coord { column: 79, row: 2 });
	assert_eq!(
		Coord::from_string("ZA2"),
		Coord {
			column: 677,
			row: 2
		}
	);
	assert_eq!(
		Coord::from_string("ZZ2"),
		Coord {
			column: 702,
			row: 2
		}
	);
	assert_eq!(
		Coord::from_string("ZZA2"),
		Coord {
			column: 703,
			row: 2
		}
	);
	assert_eq!(
		Coord::from_string("ZZB2"),
		Coord {
			column: 704,
			row: 2
		}
	);
}

impl fmt::Display for Coord {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.column, self.row)
	}
}

pub struct Postfixcsv {
	sheet_size: Coord,
}
