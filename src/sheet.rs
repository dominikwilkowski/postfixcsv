use crate::coord::Coord;

#[derive(Debug, PartialEq)]
pub struct Sheet {
	pub data: Vec<Vec<String>>,
}

impl Sheet {
	pub fn new(input: String, separator: String) -> Self {
		Self {
			data: Self::parse(input, separator),
		}
	}

	fn parse(input: String, separator: String) -> Vec<Vec<String>> {
		let input = input.replace("\r\n", "\n").replace('\r', "\n").trim().to_string();
		input
			.split('\n')
			.map(|line| line.split(&separator).map(|s| s.to_string()).collect::<Vec<String>>())
			.collect::<Vec<Vec<String>>>()
	}

	pub fn get(&self, coord: Coord) -> Option<&String> {
		self.data.get(coord.row).and_then(|cols| cols.get(coord.column))
	}

	pub fn get_mut(&mut self, coord: Coord) -> Option<&mut String> {
		self.data.get_mut(coord.row).and_then(|cols| cols.get_mut(coord.column))
	}
}

#[test]
fn parse_test() {
	assert_eq!(
		Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), String::from(",")),
		Sheet {
			data: vec![
				vec![String::from("cellA1"), String::from("cellB1"), String::from("cellC1")],
				vec![String::from("cellA2"), String::from("cellB2"), String::from("cellC2")],
				vec![String::from("cellA3"), String::from("cellB3"), String::from("cellC3")],
			]
		}
	);

	assert_eq!(
		Sheet::new(String::from("cellA1,cellB1,cellC1\r\ncellA2,cellB2,cellC2\rcellA3,cellB3,cellC3\n"), String::from(",")),
		Sheet {
			data: vec![
				vec![String::from("cellA1"), String::from("cellB1"), String::from("cellC1")],
				vec![String::from("cellA2"), String::from("cellB2"), String::from("cellC2")],
				vec![String::from("cellA3"), String::from("cellB3"), String::from("cellC3")],
			]
		}
	);

	assert_eq!(
		Sheet::new(String::from("cellA1|cellB1|cellC1\ncellA2|cellB2|cellC2\ncellA3|cellB3|cellC3\n"), String::from("|")),
		Sheet {
			data: vec![
				vec![String::from("cellA1"), String::from("cellB1"), String::from("cellC1")],
				vec![String::from("cellA2"), String::from("cellB2"), String::from("cellC2")],
				vec![String::from("cellA3"), String::from("cellB3"), String::from("cellC3")],
			]
		}
	);
}

#[test]
fn get_test() {
	let sheet =
		Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), String::from(","));

	assert_eq!(sheet.get(Coord { column: 0, row: 0 }), Some(&String::from("cellA1")));
	assert_eq!(sheet.get(Coord { column: 1, row: 1 }), Some(&String::from("cellB2")));
	assert_eq!(sheet.get(Coord { column: 2, row: 2 }), Some(&String::from("cellC3")));
	assert_eq!(sheet.get(Coord { column: 0, row: 3 }), None);
	assert_eq!(sheet.get(Coord { column: 3, row: 0 }), None);
}

#[test]
fn get_mut_test() {
	let mut sheet =
		Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), String::from(","));

	assert_eq!(sheet.get_mut(Coord { column: 0, row: 0 }), Some(&mut String::from("cellA1")));
	assert_eq!(sheet.get_mut(Coord { column: 1, row: 1 }), Some(&mut String::from("cellB2")));
	assert_eq!(sheet.get_mut(Coord { column: 2, row: 2 }), Some(&mut String::from("cellC3")));
	assert_eq!(sheet.get_mut(Coord { column: 0, row: 3 }), None);
	assert_eq!(sheet.get_mut(Coord { column: 3, row: 0 }), None);
}

pub struct SheetIterator<'a> {
	sheet: &'a Sheet,
	row_index: usize,
	col_index: usize,
}

impl Sheet {
	pub fn iter(&self) -> SheetIterator {
		SheetIterator {
			sheet: self,
			row_index: 0,
			col_index: 0,
		}
	}
}

impl<'a> IntoIterator for &'a Sheet {
	type Item = Coord;
	type IntoIter = SheetIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> Iterator for SheetIterator<'a> {
	type Item = Coord;

	fn next(&mut self) -> Option<Self::Item> {
		if self.row_index < self.sheet.data.len() && self.col_index < self.sheet.data[self.row_index].len() {
			let coords = Coord {
				column: self.col_index,
				row: self.row_index,
			};
			self.col_index += 1;
			Some(coords)
		} else if self.col_index == self.sheet.data[self.row_index].len() && self.row_index < self.sheet.data.len() - 1 {
			self.row_index += 1;
			self.col_index = 0;
			let coords = Coord {
				column: self.col_index,
				row: self.row_index,
			};
			self.col_index += 1;
			Some(coords)
		} else {
			None
		}
	}
}

#[test]
fn iterator_test() {
	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1"), String::from(","));
	let mut sheet_iter = sheet.iter();
	assert_eq!(sheet_iter.next(), Some(Coord { column: 0, row: 0 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 1, row: 0 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 2, row: 0 }));
	assert_eq!(sheet_iter.next(), None);

	let sheet =
		Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2,cellC2\ncellA3,cellB3,cellC3\n"), String::from(","));
	let mut sheet_iter = sheet.iter();
	assert_eq!(sheet_iter.next(), Some(Coord { column: 0, row: 0 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 1, row: 0 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 2, row: 0 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 0, row: 1 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 1, row: 1 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 2, row: 1 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 0, row: 2 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 1, row: 2 }));
	assert_eq!(sheet_iter.next(), Some(Coord { column: 2, row: 2 }));
	assert_eq!(sheet_iter.next(), None);

	let sheet = Sheet::new(String::from("cellA1,cellB1,cellC1\ncellA2,cellB2"), String::from(","));
	let mut cells = vec![];
	for cell in &sheet {
		cells.push(cell);
	}
	assert_eq!(
		cells,
		vec![
			Coord { column: 0, row: 0 },
			Coord { column: 1, row: 0 },
			Coord { column: 2, row: 0 },
			Coord { column: 0, row: 1 },
			Coord { column: 1, row: 1 },
		]
	);
}
