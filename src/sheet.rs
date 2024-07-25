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

	pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut String> {
		self.data.get_mut(coord.row).and_then(|cols| cols.get_mut(coord.column))
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
fn get_mut_test() {
	let mut sheet = Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), ",");

	assert_eq!(sheet.get_mut(&Coord { column: 0, row: 0 }), Some(&mut String::from("cellA1")));
	assert_eq!(sheet.get_mut(&Coord { column: 1, row: 1 }), Some(&mut String::from("cellB2")));
	assert_eq!(sheet.get_mut(&Coord { column: 2, row: 2 }), Some(&mut String::from("cellC3")));
	assert_eq!(sheet.get_mut(&Coord { column: 0, row: 3 }), None);
	assert_eq!(sheet.get_mut(&Coord { column: 3, row: 0 }), None);
}

pub struct SheetIterator<'a> {
	sheet: &'a Sheet<'a>,
	row_index: usize,
	col_index: usize,
}

impl<'a> Sheet<'a> {
	pub fn iter(&self) -> SheetIterator {
		SheetIterator {
			sheet: self,
			row_index: 0,
			col_index: 0,
		}
	}
}

impl<'a> IntoIterator for &'a Sheet<'a> {
	type Item = Coord;
	type IntoIter = SheetIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> Iterator for SheetIterator<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		if self.row_index < self.sheet.data.len() && self.col_index < self.sheet.data[self.row_index].len() {
			let cell = &self.sheet.data[self.row_index][self.col_index];
			self.col_index += 1;
			Some(cell)
		} else if self.col_index == self.sheet.data[self.row_index].len() && self.row_index < self.sheet.data.len() - 1 {
			self.row_index += 1;
			self.col_index = 0;
			let cell = &self.sheet.data[self.row_index][self.col_index];
			self.col_index += 1;
			Some(cell)
		} else {
			None
		}
	}
}

#[test]
fn iterator_test() {
	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1"), ",");
	let mut sheet_iter = sheet.iter();
	assert_eq!(sheet_iter.next(), Some("cellA1"));
	assert_eq!(sheet_iter.next(), Some("cellB1"));
	assert_eq!(sheet_iter.next(), Some("cellC1"));
	assert_eq!(sheet_iter.next(), None);

	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), ",");
	let mut sheet_iter = sheet.iter();
	assert_eq!(sheet_iter.next(), Some("cellA1"));
	assert_eq!(sheet_iter.next(), Some("cellB1"));
	assert_eq!(sheet_iter.next(), Some("cellC1"));
	assert_eq!(sheet_iter.next(), Some("cellA2"));
	assert_eq!(sheet_iter.next(), Some("cellB2"));
	assert_eq!(sheet_iter.next(), Some("cellC2"));
	assert_eq!(sheet_iter.next(), Some("cellA3"));
	assert_eq!(sheet_iter.next(), Some("cellB3"));
	assert_eq!(sheet_iter.next(), Some("cellC3"));
	assert_eq!(sheet_iter.next(), None);

	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2"), ",");
	let mut cells = vec![];
	for cell in &sheet {
		cells.push(cell);
	}
	assert_eq!(cells, vec!["cellA1", "cellB1", "cellC1", "cellA2", "cellB2",]);
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
