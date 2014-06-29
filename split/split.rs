#![crate_id(name="mkdir", vers="1.0.0", author="Akira Hayakawa")]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Akira Hayakawa <ruby.wktk@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

#![feature(macro_rules)]

extern crate getopts;
extern crate libc;

use std::os;
use std::io::{print, stdin, File, BufferedReader};
use std::num;
use std::char;

#[path = "../common/util.rs"]
mod util;

static NAME: &'static str = "split";
static VERSION: &'static str = "1.0.0";

struct Settings {
	prefix: String,
	numeric_suffix: bool,
	suffix_length: uint,
	input: String,
	strategy: String,
	strategy_param: String,
	verbose: bool
}

struct SplitControl {
	current_lineno: uint,
	current_line: String,
}

trait Splitter {
	fn new(&Settings) -> Self;
	fn consume(&mut self, &mut SplitControl) -> String;
}

struct LineSplitter {
	lines_to_write: uint,
}

impl Splitter for LineSplitter {
	fn new(settings: &Settings) -> LineSplitter {
		let n = match from_str(settings.strategy_param.as_slice()) {
			Some(a) => a,
			_ => crash!(1, "invalid number of lines")
		};
		LineSplitter {
			lines_to_write: n,
		}
	}

	fn consume(&mut self, control: &mut SplitControl) -> String {
		self.lines_to_write -= 1;
		control.current_line.clone()
	}
}
//
// struct ByteSplitter {
// }
//
// struct LineByteSplitter {
// }

// (1, 3) -> "aab"
fn str_prefix(i: uint, width: uint) -> String {
	let mut c = "".to_string();

	let mut n = i;
	let mut w = width;
	while w > 0 {
		w -= 1;
		let div = num::pow(26 as uint, w);
		let r = n / div;
		n -= r * div;
		c.push_char(char::from_u32((r as u32) + 97).unwrap());
	}
	c
}

// (1, 3) -> "001"
fn num_prefix(i: uint, width: uint) -> String {
	let mut c = "".to_string();

	let mut n = i;
	let mut w = width;
	while w > 0 {
		w -= 1;
		let div = num::pow(10 as uint, w);
		let r = n / div;
		n -= r * div;
		c.push_char(char::from_digit(r, 10).unwrap());
	}
	c
}

#[allow(dead_code)]
fn main() { os::set_exit_status(uumain(os::args())); }

pub fn uumain(args: Vec<String>) -> int {
	let opts = [
		getopts::optopt("a", "suffix-length", "use suffixes of length N (default 2)", "N"),
		getopts::optopt("b", "bytes", "put SIZE bytes per output file", "SIZE"),
		getopts::optopt("C", "line-bytes", "put at most SIZE bytes of lines per output file", "SIZE"),
		getopts::optflag("d", "numeric-suffixes", "use numeric suffixes instead of alphabetic"),
		getopts::optopt("l", "lines", "put NUMBER lines per output file", "NUMBER"),
		getopts::optflag("", "verbose", "print a diagnostic just before each output file is opened"),
		getopts::optflag("h", "help", "display help and exit"),
		getopts::optflag("V", "version", "output version information and exit"),
	];

	let matches = match getopts::getopts(args.tail(), opts) {
		Ok(m) => m,
		Err(f) => crash!(1, "{}", f)
	};

	if matches.opt_present("h") {
		println!("{} v{}", NAME, VERSION);
		println!("");
		println!("Usage:");
		println!("  {0:s} [OPTION]... [INPUT [PREFIX]]", NAME);
		println!("");
		print(getopts::usage("Output fixed-size pieces of INPUT to PREFIXaa, PREFIX ab, ...; default size is 1000, and default PREFIX is 'x'. With no INPUT, or when INPUT is -, read standard input." , opts).as_slice());
		return 0;
	}

	if matches.opt_present("V") {
		println!("{} v{}", NAME, VERSION);
		return 0;
	}

	// consume args

	let mut settings = Settings {
		prefix: "".to_string(),
		numeric_suffix: false,
		suffix_length: 0,
		input: "".to_string(),
		strategy: "".to_string(),
		strategy_param: "".to_string(),
		verbose: false,
	};
	settings.numeric_suffix = if matches.opt_present("d") { true } else { false };

	settings.suffix_length = match matches.opt_str("a") {
		Some(n) => match from_str(n.as_slice()) {
				Some(m) => m,
				None => crash!(1, "cannot parse num")
		},
		None => 2
	};

	settings.verbose = if matches.opt_present("verbose") { true } else { false };

	settings.strategy = "b".to_string();
	settings.strategy_param = "1000".to_string();
	let strategies = vec!["b", "C", "l"];
	for e in strategies.iter() {
		match matches.opt_str(*e) {
			Some(a) => {
				if settings.strategy.as_slice() == "l" {
					settings.strategy = e.to_string();
					settings.strategy_param = a;
				} else {
					crash!(1, "{}: cannot split in more than one way", NAME)
				}
			},
			None => {}
		}
	}

	let mut v = matches.free.iter();
	let (input, prefix) = match (v.next(), v.next()) {
		(Some(a), None) => (a.to_string(), "x".to_string()),
		(Some(a), Some(b)) => (a.to_string(), b.to_string()),
		(None, _) => ("-".to_string(), "x".to_string()),
	};
	settings.input = input;
	settings.prefix = prefix;

	let mut buffer = if settings.input.as_slice() == "-" { 
		BufferedReader::new(stdin());
	} else { 
		let path = Path::new(settings.input.as_slice());
		let reader = match File::open(&path) {
			Ok(a) => a,
			Err(e) => crash!(1, "cannot open '{}' for reading: No such file or directory", settings.input)
		};
		BufferedReader::new(reader);
	};

	// println!("{}", num_prefix(128, 4));
	// println!("{}", str_prefix(1, 5));

	// XXX attempt write
	// let mut splitter = LineSplitter::new(options); 
	// let mut fileno = 0;
	// while splitter.current_lineno <  spliter.no_lines {
	// 	if splitter.current_line.char_len() == 0 {
	// 		splitter.current_line = ; // slice
	// 	}
        //
	// 	if splitter.request_new_file {
	// 		writer.flush();
        //
	// 		let mut filename = options.prefix.to_string();
	// 		filename.push_str(if options.numeric_suffix {
	// 			num_prefix(fileno, options.suffix_length);
	// 		} else {
	// 			str_prefix(fileno, options.suffix_length);	
	// 		}.as_slice());
        //
	// 		fileno += 1;
	// 		writer = ;
	// 	}
        //
	// 	let consumed = splitter.consume();
	// 	writer.write_str(consumed.as_slice());
        //
	// 	let advance = consumed.as_slice().char_len();
	// 	// advance current_line as slice
	// }

	0
}
