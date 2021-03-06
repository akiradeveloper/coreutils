#![crate_name = "groups"]
/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Alan Andrade <alan.andradec@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 */
#![feature(macro_rules)]

extern crate getopts;

use getopts::{
    optflag,
    getopts,
    usage
};
use c_types::{get_pw_from_args, group};

#[path = "../common/util.rs"] mod util;
#[path = "../common/c_types.rs"] mod c_types;

static NAME: &'static str = "groups";
static VERSION: &'static str = "1.0.0";

pub fn uumain(args: Vec<String>) -> int {
    let program = args.get(0).clone();

    let options = [
        optflag("h", "help", "display this help menu and exit"),
        optflag("V", "version", "display version information and exit")
    ];

    let matches = match getopts(args.tail(), options) {
        Ok(m) => { m },
        Err(f) => {
            show_error!("{}", f);
            return 1;
        }
    };

    if matches.opt_present("version") {
        println!("{} v{}", NAME, VERSION);
    } else if matches.opt_present("help") {
        print!("{} v{}\n\n\
                Usage:\n  \
                  {} [OPTION]... [USER]...\n\n\
                {}", NAME, VERSION, program, usage("Prints the groups a user is in to standard output.", options));
    } else {
        group(get_pw_from_args(&matches.free), true);
    }

    0
}
