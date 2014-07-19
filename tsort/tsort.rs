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
use std::collections::{HashSet, HashMap};
use std::io::{print};

#[path = "../common/util.rs"]
mod util;

static NAME: &'static str = "tsort";
static VERSION: &'static str = "1.0.0";

pub fn uumain(args: Vec<String>) -> int {
	let prog_name = args.get(0).clone();
	let opts = [
		getopts::optflag("h", "help", "display this help and exit"),
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
        println!("	{} [OPTIONS] FILE", NAME);
        println!("");
        io::print(getopts::usage("Topological sort the strings in FILE. Strings are defined as any sequence of tokens separated by whitespace (tab, space, or newline). If FILE is not passed in, stdin is used instead.", opts).as_slice());
    	return 0;
    }

    if matches.opt_present("V") {
        println!("{} v{}", NAME, VERSION);
        return 0;
    }

    let mut files = matches.free.clone();
    let input = if files.len() > 1 {
        crash!(1, "{}, extra operand '{}'", NAME, matches.free[1]);
    } else if (files.is_empty()) {
        "-".to_string()
    } else {
        files[0].to_string()
    };

    let mut reader = io::BufferedReader::new(
        if input.as_slice() == "-" {
            box io::stdio::stdin_raw() as Box<Reader>
        } else {
            box match io::File::open(&Path::new(input.clone())) {
                Ok(a) => a,
                _ => crash!(1, "{}: No such file or directory", input)
            } as Box<Reader>
        }
    );

    let mut g = Graph::new();

    // TODO init g

    let mut res = g.run_tsort();

    let mut writer = io::BufferedWriter::new(box io::stdio::stdout_raw() as Box<Writer>);

	return 0
}

struct Edge {
    in_edges: HashSet<String>,
    out_edges: HashSet<String>
}

impl Edge {
    fn new() -> Edge {
        Edge {
            in_edges: HashSet::new(),
            out_edges: HashSet::new()
        }
    }
}

struct Graph {
    nodes: Vec<String>, // Ordered
    edges: HashMap<String, Edge>,
    result: Vec<String> // Ordered
}

// Kahn's algorithm
impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: vec!(),
            edges: HashMap::new(),
            result: vec!(),
        }
    }

    fn add_edge(&mut self, from: String,  to: String) {
        if self.edges.contains_key(&from) {
            self.edges.insert(from.clone(), Edge::new());
        }
        if self.edges.contains_key(&to) {
            self.edges.insert(to.clone(), Edge::new());
        }

        self.edges.get_mut(&from).out_edges.insert(to.clone());
        self.edges.get_mut(&to).out_edges.insert(from.clone());
    }

    fn run_tsort(&mut self) {
    }

    fn is_acyclic(&self) -> bool {
        false 
    }
}
