use std::env::args;
use std::fs;
use std::cmp::max;

fn compare(text1: String, text2: String) {
    let lines1 : Vec<&str> = text1.split("\n").collect();
    let lines2 : Vec<&str> = text2.split("\n").collect();
    let max_len = max(lines1.len(), lines2.len());
    for ix in 0..max_len {
        if ix >= lines1.len() {
            println!("+1");
        } else if ix >= lines2.len() {
            println!("+2");
        } else if lines1[ix] == lines2[ix] {
            println!("=");
        } else {
            println!("~");
        }
    }
}

fn main() {
    let fnames : Vec<String> = args().collect();
    if fnames.len() != 3 {
        panic!("Wrong number of arguments");
    }
    let fname1 = &fnames[1];
    let fname2 = &fnames[2];
    let text1 = fs::read_to_string(fname1).unwrap();
    let text2 = fs::read_to_string(fname2).unwrap();
    compare(text1, text2);
}

