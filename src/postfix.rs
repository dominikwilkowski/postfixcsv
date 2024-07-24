use std::fmt;

use crate::{coord::Coord, sheet::Sheet};

#[derive(Debug, PartialEq)]
pub enum PostfixError {
	RecursionDepthExceeded,
	NotEnoughOperands,
	TooManyOperands,
	CellNotFound,
	DivisionByZero,
}

impl fmt::Display for PostfixError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "#ERR")
	}
}

#[test]
fn error_enum_test() {
	assert_eq!(format!("{}", PostfixError::RecursionDepthExceeded), String::from("#ERR"));
	assert_eq!(format!("{}", PostfixError::NotEnoughOperands), String::from("#ERR"));
	assert_eq!(format!("{}", PostfixError::TooManyOperands), String::from("#ERR"));
	assert_eq!(format!("{}", PostfixError::CellNotFound), String::from("#ERR"));
	assert_eq!(format!("{}", PostfixError::DivisionByZero), String::from("#ERR"));
}

#[derive(Debug, PartialEq)]
pub struct Postfix<'a> {
	pub sheet: &'a mut Sheet<'a>,
	stack: Vec<f64>,
}

impl<'a> Postfix<'a> {
	pub fn new(sheet: &'a mut Sheet<'a>) -> Self {
		Self {
			sheet,
			stack: Vec::new(),
		}
	}

	fn sanatize_input(input: &str) -> Vec<String> {
		input.trim().to_string().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
	}

	pub fn calc_cell(&mut self, coord: &Coord, recursion_depth: u8) -> Result<f64, PostfixError> {
		if recursion_depth == 255 {
			return Err(PostfixError::RecursionDepthExceeded);
		} else if recursion_depth == 0 {
			self.stack.clear();
		}

		if let Some(cell) = self.sheet.get(coord) {
			let cell = Self::sanatize_input(cell);

			for item in &cell {
				let cell = if Coord::is_coord(item) {
					let coord = Coord::parse(item);
					let calc_result = self.calc_cell(&coord, recursion_depth + 1);
					match calc_result {
						Ok(value) => &value.to_string(),
						Err(err) => return Err(err),
					}
				} else {
					item
				};

				match cell.as_str() {
					"+" => {
						let (a, b) = (self.stack.pop(), self.stack.pop());
						if let (Some(a), Some(b)) = (a, b) {
							self.stack.push(b + a);
						} else {
							return Err(PostfixError::NotEnoughOperands);
						}
					},
					"-" => {
						let (a, b) = (self.stack.pop(), self.stack.pop());
						if let (Some(a), Some(b)) = (a, b) {
							self.stack.push(b - a);
						} else {
							return Err(PostfixError::NotEnoughOperands);
						}
					},
					"*" => {
						let (a, b) = (self.stack.pop(), self.stack.pop());
						if let (Some(a), Some(b)) = (a, b) {
							self.stack.push(b * a);
						} else {
							return Err(PostfixError::NotEnoughOperands);
						}
					},
					"/" => {
						let (a, b) = (self.stack.pop(), self.stack.pop());
						if let (Some(a), Some(b)) = (a, b) {
							if a == 0.0 {
								return Err(PostfixError::DivisionByZero);
							}
							self.stack.push(b / a);
						} else {
							return Err(PostfixError::NotEnoughOperands);
						}
					},
					_ => {
						if let Ok(operand) = item.parse::<f64>() {
							self.stack.push(operand);
						}
					},
				}
			}

			if recursion_depth > 0 && (self.stack.len() > 1 || self.stack.is_empty()) {
				// we are inside a recursive call
				Ok(*self.stack.last().unwrap())
			} else if self.stack.len() > 1 {
				Err(PostfixError::TooManyOperands)
			} else if self.stack.is_empty() {
				Err(PostfixError::NotEnoughOperands)
			} else {
				Ok(self.stack[0])
			}
		} else {
			Err(PostfixError::CellNotFound)
		}
	}

	pub fn process_sheet(&mut self) {
		let sheet_coords = self.sheet.iter().collect::<Vec<Coord>>();

		for cell_coord in sheet_coords {
			let value = match self.calc_cell(&cell_coord, 0) {
				Ok(result) => format!("{result}"),
				Err(error) => format!("{error}"),
			};

			if let Some(cell) = self.sheet.get_mut(&cell_coord) {
				*cell = value;
			}
		}
	}
}

#[test]
fn new_test() {
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("x x")]],
			separator: ",",
		}),
		Postfix {
			sheet: &mut Sheet {
				data: vec![vec![String::from("x x")]],
				separator: ",",
			},
			stack: Vec::new(),
		}
	);
}

#[test]
fn calc_cell_test() {
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from(" 2    3 + ")]],
			separator: ","
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("2 3 +")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("2 5 3 * -")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Ok(-13.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("6 2 /")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Ok(3.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from(
				"          8 ? 19.5 # 6  / : * 3  1.5  14 - +      * "
			)]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Ok(-247.0)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("6 /")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("62/")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("6 2 / +")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("6 /")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("?")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5"), String::from("A1 2 +")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Ok(7.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5"), String::from("A1 C1 +"), String::from("2")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Ok(7.0)
	);

	let data = vec![
		vec![String::from("B1 B2 +"), String::from("2 B2 3 * -"), String::from("+")],
		vec![String::from("A1"), String::from("5"), String::from("7 2 /")],
		vec![
			String::from("C2 3 *"),
			String::from("1 2"),
			String::from("5 1 2 + 4 * + 3 -"),
		],
	];
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 0, row: 0 }, 0),
		Ok(-8.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Ok(-13.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 2, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 0, row: 1 }, 0),
		Ok(-8.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 1, row: 1 }, 0),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 2, row: 1 }, 0),
		Ok(3.5)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 0, row: 2 }, 0),
		Ok(10.5)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell(&Coord { column: 1, row: 2 }, 0),
		Err(PostfixError::TooManyOperands)
	);
	assert_eq!(Postfix::new(&mut Sheet { data, separator: "," }).calc_cell(&Coord { column: 2, row: 2 }, 0), Ok(14.0));

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("B1"), String::from("A1 C1 +"), String::from("2")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Err(PostfixError::RecursionDepthExceeded)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5 5"), String::from("5 C1 +"), String::from("A1")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Err(PostfixError::TooManyOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5"), String::from("D1 C1 +"), String::from("A1")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Err(PostfixError::CellNotFound)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5"), String::from("A2 C1 +"), String::from("A1")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Err(PostfixError::CellNotFound)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5 -"), String::from("4 C1 +"), String::from("A1")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("0"), String::from("4 C1 /"), String::from("A1")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Err(PostfixError::DivisionByZero)
	);

	// A quirk I'll call a "feature", not a "bug"
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5"), String::from("A1 C1 + +"), String::from("A1 1")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Ok(11.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5"), String::from("A1 5 1 C1"), String::from("+ +")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 1, row: 0 }, 0),
		Ok(11.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec![String::from("5"), String::from("A1 5 1 B1"), String::from("+ +")]],
			separator: ",",
		})
		.calc_cell(&Coord { column: 2, row: 0 }, 0),
		Err(PostfixError::NotEnoughOperands)
	);
}

#[test]
fn process_sheet_test() {
	let data = vec![
		vec![String::from("B1 B2 +"), String::from("2 B2 3 * -"), String::from("+")],
		vec![String::from("A1"), String::from("5"), String::from("7 2 /")],
		vec![
			String::from("C2 3 *"),
			String::from("1 2"),
			String::from("5 1 2 + 4 * + 3 -"),
		],
	];
	let mut sheet = Sheet { data, separator: "," };
	let mut postfix = Postfix::new(&mut sheet);
	postfix.process_sheet();

	assert_eq!(
		postfix.sheet,
		&mut Sheet {
			data: vec![
				vec![String::from("-8"), String::from("-13"), String::from("#ERR")],
				vec![String::from("-8"), String::from("5"), String::from("3.5")],
				vec![String::from("10.5"), String::from("#ERR"), String::from("14"),],
			],
			separator: ","
		}
	);
}
