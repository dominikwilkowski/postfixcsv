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
