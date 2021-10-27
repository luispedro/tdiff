#![feature(iter_intersperse)]

use std::env::args;
use std::fs;
pub mod compare;
pub mod parse;

fn main() {
    let fnames : Vec<String> = args().collect();
    if fnames.len() != 3 {
        panic!("Wrong number of arguments");
    }
    let fname1 = &fnames[1];
    let fname2 = &fnames[2];
    let text1 = fs::read_to_string(fname1).unwrap();
    let text2 = fs::read_to_string(fname2).unwrap();
    compare::compare(text1, text2);
}

