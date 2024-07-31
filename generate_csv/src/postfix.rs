use rand::Rng;
use std::collections::VecDeque;

use process_csv::coord::Coord;

fn precedence(op: char) -> i32 {
	match op {
		'+' | '-' => 1,
		'*' | '/' => 2,
		_ => 0,
	}
}

pub fn infix_2_postfix(expression: String, rows: usize, cols: usize) -> String {
	let mut output = VecDeque::new();
	let mut operators = Vec::new();

	for token in expression.chars() {
		if token.is_whitespace() {
			continue;
		}
		if token.is_digit(10) {
			let mut rng = rand::thread_rng();
			if rng.gen_range(1..40) == 7 {
				output.push_back(
					Coord {
						column: rng.gen_range(1..=(cols - 1)),
						row: rng.gen_range(1..=(rows - 1)),
					}
					.stringify(),
				);
			} else {
				output.push_back(token.to_string());
			}
		} else if token == '(' {
			operators.push(token);
		} else if token == ')' {
			while let Some(&top) = operators.last() {
				if top == '(' {
					break;
				}
				output.push_back(operators.pop().unwrap().to_string());
			}
			operators.pop();
		} else {
			while let Some(&top) = operators.last() {
				if precedence(top) >= precedence(token) {
					output.push_back(operators.pop().unwrap().to_string());
				} else {
					break;
				}
			}
			operators.push(token);
		}
	}

	while let Some(op) = operators.pop() {
		output.push_back(op.to_string());
	}

	output.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ")
}
