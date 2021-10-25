use std::env::args;
use std::fs;

fn main() {
    let fnames : Vec<String> = args().collect();
    if fnames.len() != 3 {
        panic!("Wrong number of arguments");
    }
    let fname1 = &fnames[1];
    let fname2 = &fnames[2];
    let text1 = fs::read_to_string(fname1).unwrap();
    let text2 = fs::read_to_string(fname2).unwrap();
    println!("Number of characters (1): {}", text1.len());
    println!("Number of characters (2): {}", text2.len());
}

