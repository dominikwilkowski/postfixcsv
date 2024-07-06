use std::env;

pub mod args;
pub mod coord;
pub mod postfixcsv;

use crate::args::Args;

fn main() {
	let cli_args = Args::parse(env::args().skip(1).collect::<Vec<String>>());

	println!("Hello, world! {cli_args:?}");
}
