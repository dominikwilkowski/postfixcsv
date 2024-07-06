use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Args {
	csv_path: PathBuf,
	separator: String,
	help: bool,
	version: bool,
}

impl Args {
	pub fn parse(args: Vec<String>) -> Self {
		let mut flags = Self {
			csv_path: PathBuf::new(),
			separator: String::from(","),
			help: false,
			version: false,
		};

		let mut iter = args.iter().peekable();
		while let Some(arg) = iter.next() {
			match arg.as_str() {
				"-v" | "-V" | "--version" => {
					flags.version = true;
				},
				"-h" | "--help" => {
					flags.help = true;
				},
				"-s" | "--separator" => {
					if let Some(next_arg) = iter.next() {
						flags.separator.clone_from(next_arg);
					} else {
						panic!("The flag `--separator` is missing it's argument to specifiy what the separator is")
					}
				},
				_ => {
					if !arg.starts_with("-") {
						flags.csv_path.push(arg)
					}
				},
			}
		}

		if flags.csv_path.as_path().components().count() == 0 {
			panic!("Please specifiy the path to the csv to be parsed")
		}

		flags
	}
}

#[test]
fn parse_test() {
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-v")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: false,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-V")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: false,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("--version")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: false,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-h")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: true,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("--help")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: true,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("--unknown")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-x")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from(","),
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-s"), String::from(".")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from("."),
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("path/to/somehwere"),
			String::from("--separator"),
			String::from(".")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from("."),
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("-v"),
			String::from("path/to/somehwere"),
			String::from("-s"),
			String::from("..."),
			String::from("-w"),
			String::from("-h"),
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from("..."),
			help: true,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("-v"),
			String::from("-s"),
			String::from("..."),
			String::from("-h"),
			String::from("-w"),
			String::from("path/to/somehwere")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from("..."),
			help: true,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("path/to/somehwere"),
			String::from("--version"),
			String::from("--help"),
			String::from("--separator"),
			String::from("..."),
			String::from("--windows"),
			String::from("path/to/elsewhere")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere/path/to/elsewhere"),
			separator: String::from("..."),
			help: true,
			version: true,
		}
	);
}

#[test]
#[should_panic]
fn parse_test_panic() {
	assert_eq!(
		Args::parse(vec![]),
		Args {
			csv_path: PathBuf::new(),
			separator: String::from(","),
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-s"),]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: String::from("..."),
			help: false,
			version: true,
		}
	);
}
