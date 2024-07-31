use std::{
	fs::File,
	io::{self, Write},
	time::Instant,
};

mod infix;
mod postfix;

use crate::{infix::generate_random_expression, postfix::infix_2_postfix};

const ROWS: usize = 10_000;
const COLS: usize = 10_000;
const OUTPUT_FILE_NAME: &'static str = "./postfix.csv";

fn main() -> io::Result<()> {
	let time = Instant::now();
	let num_expressions = ROWS * COLS;
	let max_depth = 3;
	let mut file =
		File::create(OUTPUT_FILE_NAME).expect("Creating write lock for output file {OUTPUT_FILE_NAME:?} failed.");

	for i in 0..num_expressions {
		if i % COLS == 0 && i != 0 {
			write!(file, "\n")?;
		} else if i % COLS != 0 {
			write!(file, ",")?;
		}

		let expression = generate_random_expression(max_depth);
		let postfix = infix_2_postfix(expression, ROWS, COLS);
		write!(file, "{}", postfix)?;
	}

	println!("File successfully written to {OUTPUT_FILE_NAME:?}\nTime: {:#?}", time.elapsed());

	Ok(())
}
