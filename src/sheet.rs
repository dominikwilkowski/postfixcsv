use std::fmt;

use crate::coord::Coord;

#[derive(Debug, PartialEq)]
pub struct Sheet<'a> {
	pub data: Vec<Vec<&'a str>>,
	pub separator: &'a str,
}

impl<'a> Sheet<'a> {
	pub fn new(input: &'a str, separator: &'a str) -> Self {
		Self {
			data: Self::parse(input, separator),
			separator,
		}
	}

	fn parse(input: &'a str, separator: &'a str) -> Vec<Vec<&'a str>> {
		input.trim().lines().map(|line| line.split(&separator).collect::<Vec<&'a str>>()).collect::<Vec<Vec<&'a str>>>()
	}

	pub fn get(&self, coord: &Coord) -> Option<&str> {
		self.data.get(coord.row).and_then(|cols| cols.get(coord.column)).copied()
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
		Sheet::new(&"cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n", ","),
		Sheet {
			data: vec![
				vec!["cellA1", "cellB1", "cellC1"],
				vec!["cellA2", "cellB2", "cellC2"],
				vec!["cellA3", "cellB3", "cellC3"],
			],
			separator: ","
		}
	);

	assert_eq!(
		Sheet::new("cellA1,cellB1,cellC1\r\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n", ","),
		Sheet {
			data: vec![
				vec!["cellA1", "cellB1", "cellC1"],
				vec!["cellA2", "cellB2", "cellC2"],
				vec!["cellA3", "cellB3", "cellC3"],
			],
			separator: ","
		}
	);

	assert_eq!(
		Sheet::new("cellA1|cellB1|cellC1\ncellA2|cellB2|cellC2\ncellA3|cellB3|cellC3\n", "|"),
		Sheet {
			data: vec![
				vec!["cellA1", "cellB1", "cellC1"],
				vec!["cellA2", "cellB2", "cellC2"],
				vec!["cellA3", "cellB3", "cellC3"],
			],
			separator: "|"
		}
	);
}

#[test]
fn get_test() {
	let sheet = Sheet::new("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n", ",");

	assert_eq!(sheet.get(&Coord { column: 0, row: 0 }), Some("cellA1"));
	assert_eq!(sheet.get(&Coord { column: 1, row: 1 }), Some("cellB2"));
	assert_eq!(sheet.get(&Coord { column: 2, row: 2 }), Some("cellC3"));
	assert_eq!(sheet.get(&Coord { column: 0, row: 3 }), None);
	assert_eq!(sheet.get(&Coord { column: 3, row: 0 }), None);
}

#[test]
fn display_test() {
	let sheet = Sheet::new("cellA1,cellB1,cellC1\ncellA2,cellB2", ",");
	assert_eq!(format!("{sheet}"), "cellA1,cellB1,cellC1\ncellA2,cellB2");

	let sheet = Sheet::new("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3", ",");
	assert_eq!(format!("{sheet}"), "cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3");

	let sheet = Sheet::new("cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2", ";");
	assert_eq!(format!("{sheet}"), "cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2");

	let sheet = Sheet::new("cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2\n\n", ";");
	assert_eq!(format!("{sheet}"), "cellA1;cellB1;cellC1\ncellA2;cellB2;cellC2");
}
