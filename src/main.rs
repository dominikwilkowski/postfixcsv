use std::env;

pub mod coord;
pub mod postfixcsv;

fn main() {
	let cli_args = env::args().skip(1).collect::<Vec<String>>();
	// parse cli args for three options:
	// - path for CSV
	// - flag for separator `-s "|"` or `--separator ,`
	// - -v, -V or --version

	println!("Hello, world! {cli_args:?}");
}
