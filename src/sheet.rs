use std::fmt;

use crate::coord::Coord;

#[derive(Debug, PartialEq)]
pub struct Sheet<'a> {
	pub data: Vec<Vec<String>>,
	pub separator: &'a str,
}

impl<'a> Sheet<'a> {
	pub fn new(input: String, separator: &'a str) -> Self {
		Self {
			data: Self::parse(input, separator),
			separator,
		}
	}

	fn parse(input: String, separator: &str) -> Vec<Vec<String>> {
		let input = input.replace("\r\n", "\n").replace('\r', "\n").trim().to_string();
		input
			.split('\n')
			.map(|line| line.split(&separator).map(|s| s.to_string()).collect::<Vec<String>>())
			.collect::<Vec<Vec<String>>>()
	}

	pub fn get(&self, coord: &Coord) -> Option<&String> {
		self.data.get(coord.row).and_then(|cols| cols.get(coord.column))
	}
}

impl<'a> fmt::Display for Sheet<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut vec = Vec::new();
		for row in &self.data {
			vec.push(row.join(self.separator));
		}
		write!(f, "{}", vec.join("\n"))?;
		Ok(())
	}
}

#[test]
fn parse_test() {
	assert_eq!(
		Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), ","),
		Sheet {
			data: vec![
				vec![String::from("cellA1"), String::from("cellB1"), String::from("cellC1")],
				vec![String::from("cellA2"), String::from("cellB2"), String::from("cellC2")],
				vec![String::from("cellA3"), String::from("cellB3"), String::from("cellC3")],
			],
			separator: ","
		}
	);

	assert_eq!(
		Sheet::new(String::from("cellA1,cellB1,cellC1\r\ncellA2,cellB2,cellC2\rcellA3,cellB3,cellC3\n"), ","),
		Sheet {
			data: vec![
				vec![String::from("cellA1"), String::from("cellB1"), String::from("cellC1")],
				vec![String::from("cellA2"), String::from("cellB2"), String::from("cellC2")],
				vec![String::from("cellA3"), String::from("cellB3"), String::from("cellC3")],
			],
			separator: ","
		}
	);

	assert_eq!(
		Sheet::new(String::from("cellA1|cellB1|cellC1\ncellA2|cellB2|cellC2\ncellA3|cellB3|cellC3\n"), "|"),
		Sheet {
			data: vec![
				vec![String::from("cellA1"), String::from("cellB1"), String::from("cellC1")],
				vec![String::from("cellA2"), String::from("cellB2"), String::from("cellC2")],
				vec![String::from("cellA3"), String::from("cellB3"), String::from("cellC3")],
			],
			separator: "|"
		}
	);
}

#[test]
fn get_test() {
	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), ",");

	assert_eq!(sheet.get(&Coord { column: 0, row: 0 }), Some(&String::from("cellA1")));
	assert_eq!(sheet.get(&Coord { column: 1, row: 1 }), Some(&String::from("cellB2")));
	assert_eq!(sheet.get(&Coord { column: 2, row: 2 }), Some(&String::from("cellC3")));
	assert_eq!(sheet.get(&Coord { column: 0, row: 3 }), None);
	assert_eq!(sheet.get(&Coord { column: 3, row: 0 }), None);
}

#[test]
fn display_test() {
	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2"), ",");
	assert_eq!(format!("{sheet}"), String::from("cellA1,cellB1,cellC1\ncellA2,cellB2"));

	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3"), ",");
	assert_eq!(format!("{sheet}"), String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3"));

	let sheet = Sheet::new(String::from("cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2"), ";");
	assert_eq!(format!("{sheet}"), String::from("cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2"));

	let sheet = Sheet::new(String::from("cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2\n\n"), ";");
	assert_eq!(format!("{sheet}"), String::from("cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2"));
}
