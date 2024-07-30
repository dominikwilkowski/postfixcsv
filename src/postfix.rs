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

	fn sanatize_input(input: &str) -> Vec<&str> {
		input.split_whitespace().collect::<Vec<&str>>()
	}

	pub fn calc_cell(&self, cell: &str, recursion_depth: u8) -> Result<f64, PostfixError> {
		let mut stack = Vec::new();

		if recursion_depth == 255 {
			return Err(PostfixError::RecursionDepthExceeded);
		}

		let cell = Self::sanatize_input(cell);
		for item in &cell {
			match *item {
				"+" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						stack.push(b + a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				"-" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						stack.push(b - a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				"*" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						stack.push(b * a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				"/" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						if a == 0.0 {
							return Err(PostfixError::DivisionByZero);
						}
						stack.push(b / a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				_ => {
					if Coord::is_coord(item) {
						let coord = Coord::parse(item);

						match self.sheet.get(&coord) {
							Some(contents) => {
								match self.calc_cell(contents, recursion_depth + 1) {
									Ok(calc_cell) => stack.push(calc_cell),
									Err(error) => return Err(error),
								};
							},
							None => return Err(PostfixError::CellNotFound),
						}
					} else if let Ok(operand) = item.parse::<f64>() {
						stack.push(operand);
					}
				},
			}
		}

		if recursion_depth > 0 && stack.len() == 1 {
			// we are inside a recursive call
			Ok(*stack.last().unwrap())
		} else if stack.len() > 1 {
			Err(PostfixError::TooManyOperands)
		} else if stack.is_empty() {
			Err(PostfixError::NotEnoughOperands)
		} else {
			Ok(stack[0])
		}
	}

	pub fn process_sheet(&self) -> Vec<Vec<String>> {
		let mut output = Vec::new();

		for row in 0..self.sheet.data.len() {
			output.push(Vec::new());
			for col in 0..self.sheet.data[row].len() {
				let cell = &self.sheet.data[row][col];

				let value = match self.calc_cell(cell, 0) {
					Ok(result) => result.to_string(),
					Err(error) => error.to_string(),
				};

				output[row].push(value);
			}
		}
		output
	}
}

#[test]
fn new_test() {
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["x x"]],
			separator: ",",
		}),
		Postfix {
			sheet: &mut Sheet {
				data: vec![vec!["x x"]],
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
			data: Vec::new(),
			separator: ","
		})
		.calc_cell(" 2    3 + ", 0),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("2 3 +", 0),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("2 5 3 * -", 0),
		Ok(-13.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("6 2 /", 0),
		Ok(3.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("          8 ? 19.5 # 6  / : * 3  1.5  14 - +      * ", 0),
		Ok(-247.0)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("6 /", 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("62/", 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("6 2 / +", 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("6 /", 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ",",
		})
		.calc_cell("?", 0),
		Err(PostfixError::NotEnoughOperands)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 2 +"]],
			separator: ",",
		})
		.calc_cell("A1 2 +", 0),
		Ok(7.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 C1 +", "2"]],
			separator: ",",
		})
		.calc_cell("A1 C1 +", 0),
		Ok(7.0)
	);

	let data = vec![
		vec!["B1 B2 +", "2 B2 3 * -", "+"],
		vec!["A1", "5", "7 2 /"],
		vec!["C2 3 *", "1 2", "5 1 2 + 4 * + 3 -"],
	];
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("B1 B2 +", 0),
		Ok(-8.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("2 B2 3 * -", 0),
		Ok(-13.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("+", 0),
		Err(PostfixError::NotEnoughOperands)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("A1", 0),
		Ok(-8.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("5", 0),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("7 2 /", 0),
		Ok(3.5)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("C2 3 *", 0),
		Ok(10.5)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ","
		})
		.calc_cell("1 2", 0),
		Err(PostfixError::TooManyOperands)
	);
	assert_eq!(Postfix::new(&mut Sheet { data, separator: "," }).calc_cell("5 1 2 + 4 * + 3 -", 0), Ok(14.0));

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["B1", "A1 C1 +", "2"]],
			separator: ",",
		})
		.calc_cell("A1 C1 +", 0),
		Err(PostfixError::RecursionDepthExceeded)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5 5", "5 C1 +", "A1"]],
			separator: ",",
		})
		.calc_cell("5 C1 +", 0),
		Err(PostfixError::TooManyOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "D1 C1 +", "A1"]],
			separator: ",",
		})
		.calc_cell("D1 C1 +", 0),
		Err(PostfixError::CellNotFound)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A2 C1 +", "A1"]],
			separator: ",",
		})
		.calc_cell("A2 C1 +", 0),
		Err(PostfixError::CellNotFound)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5 -", "4 C1 +", "A1"]],
			separator: ",",
		})
		.calc_cell("4 C1 +", 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["0", "4 C1 /", "A1"]],
			separator: ",",
		})
		.calc_cell("4 C1 /", 0),
		Err(PostfixError::DivisionByZero)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 C1 + +", "A1 1"]],
			separator: ",",
		})
		.calc_cell("A1 C1 + +", 0),
		Err(PostfixError::TooManyOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 5 1 C1", "+ +"]],
			separator: ",",
		})
		.calc_cell("A1 5 1 C1", 0),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 5 1 C1", "+ +"]],
			separator: ",",
		})
		.calc_cell("A1 5 1 C1", 0),
		Err(PostfixError::NotEnoughOperands)
	);
}

#[test]
fn process_sheet_test() {
	let data = vec![
		vec!["B1 B2 +", "2 B2 3 * -", "+"],
		vec!["A1", "5", "7 2 /"],
		vec!["C2 3 *", "1 2", "5 1 2 + 4 * + 3 -"],
	];
	let mut sheet = Sheet { data, separator: "," };
	let postfix = Postfix::new(&mut sheet);
	let output = postfix.process_sheet();

	assert_eq!(
		output,
		vec![
			vec!["-8", "-13", "#ERR"],
			vec!["-8", "5", "3.5"],
			vec!["10.5", "#ERR", "14",],
		]
	);
}
