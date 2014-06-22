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
use std::io::print;

#[path = "../common/util.rs"]
mod util;

static NAME: &'static str = "split";
static VERSION: &'static str = "1.0.0";

#[allow(dead_code)]
fn main() { os::set_exit_status(uumain(os::args())); }

pub fn uumain(args: Vec<String>) -> int {
	let opts = [
		getopts::optopt("a", "suffix-length", "use suffixes of length N (default 2)", "N"),
		getopts::optopt("b", "bytes", "put SIZE bytes per output file", "SIZE"),
		getopts::optopt("C", "line-bytes", "put at most SIZE bytes of lines per output file", "SIZE"),
		getopts::optflag("d", "numeric-suffixes", "use numeric suffixes instead of alphabetic"),
		getopts::optopt("l", "lines", "put NUMBER lines per output file", "NUMBER"),
		getopts::optflag("V", "verbose", "print a diagnostic just before each output file is opened"),
		getopts::optflag("h", "help", "display help and exit"),
		getopts::optflag("", "version", "output version information and exit"),
	];

	let matches = match getopts::getopts(args.tail(), opts) {
		Ok(m) => m,
		Err(f) => crash!(1, "{}", f.to_err_msg())
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

	let suffix_length = match matches.opt_str("a") {
		Some(n) => match from_str(n.as_slice()) {
				Some(m) => m,
				None => crash!(1, "cannot parse num")
		},
		None => 2
	};

	// XXX
	let v1 = vec!["b", "C", "l"];
	let mut v2 = v1.iter().filter_map(|x| matches.opt_str(*x));
	let strat = match (v2.next(), v2.next()) {
		(Some(_), Some(_)) => crash!(1, "{}: cannot split in more than one way", NAME),
		(Some(a), None) => a,
		(None, _) => "b".to_string(),
	};
	println!("starategy:{}", strat);

	if matches.opt_present("verbose") {
		// TODO
	}

	let mut v = matches.free.iter();
	let (input, prefix) = match (v.next(), v.next()) {
		(Some(a), None) => (a.as_slice(), "x"),
		(Some(a), Some(b)) => (a.as_slice(), b.as_slice()),
		(None, _) => ("-", "x"),
	};
	println!("input:{}, prefix:{}", input, prefix);

	0
}
