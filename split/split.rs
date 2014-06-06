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

#[allow(dead_code)]
fn main() { uumain(os::args()); }

pub fn uumain(args: Vec<String>) {
}
