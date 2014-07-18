#![crate_name = "tsort"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Ben Eggers <ben.eggers36@gmail.com>
 * (c) Akira Hayakawa <ruby.wktk@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

#![feature(macro_rules)]

extern crate getopts;
extern crate libc;

use std::os;
use std::io;

use std::io::{print};
use StdResult = std::result::Result;

#[path = "../common/util.rs"]
mod util;

static NAME: &'static str = "tsort";
static VERSION: &'static str = "1.0.0";

struct G {
    nodes: ,
    in_edges: ,
    out_edges: ,
}

impl G {
    fn new() -> Edges {
    }

    fn add_edge(from: String,  to: String) {
    }

    fn run() -> list {
    }

    fn is_acyclic() -> bool {
    
    }
}

pub fn uumain(args: Vec<String>) -> int {
	let prog_name = args.get(0).clone();
	let opts = [
		getopts::optflag("d", "debug", "print out information as the sort happens"), // FIXME remove
		getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
	];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(_) => {
            usage(prog_name, opts);
            return 1; // FIXME crash
        }
    };

    if matches.opt_present("h") {
    	usage(prog_name, opts);
    	return 0;
    }

    if matches.opt_present("V") {
        println!("{} v{}", NAME, VERSION);
        return 0;
    }

    let mut files = matches.free.clone();
    if files.is_empty() {
        files = vec!("-".to_string());
    } else if files.len() > 1 {
    	println!("{}: extra operand '{}'", prog_name, files.get(1)); // FIXME
    	return 1;
    }

    let mut reader = match open(files.get(0).to_string()) { // FIXME use [i] operator
        Ok(f) => f,
        Err(_) => { return 1; } // FIXME
    };

    let mut g = G::new();

    // TODO init g

    let mut res = g.run();

    let mut writer = io::BufferedWriter::new(box io::stdio::stdout_raw() as Box<Writer>);

	return 0
}

// FIXME remove
fn usage(prog_name: String, opts: [getopts::OptGroup, ..3]) {
    println!("Usage:");
	println!("	{} [OPTIONS] FILE", prog_name);
	print!("Topological sort the strings in FILE. "); // FIXME oneline. don't split
	print!("Strings are defined as any sequence of tokens separated by whitespace ");
	print!("(tab, space, or newline). If FILE is not passed in, stdin is used instead.");
	print(getopts::usage("", opts).as_slice());
	println!("");
}

// FIXME remove. expand
// FIXME more specific name
fn open(path: String) -> StdResult<io::BufferedReader<Box<Reader>>, int> {
    if  path.as_slice() == "-" {
        let reader = box io::stdio::stdin_raw() as Box<Reader>;
        return Ok(io::BufferedReader::new(reader));
    }
    match io::File::open(&std::path::Path::new(path.as_slice())) {
        Ok(fd) => {
            let reader = box fd as Box<Reader>;
            Ok(io::BufferedReader::new(reader))
        },
        Err(_) => { // FIXME crash
            Err(1)
        }
    }
}
