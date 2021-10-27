use std::cmp::max;

pub fn compare(text1: String, text2: String) {
    let lines1 : Vec<String> = super::parse::sentences(text1);
    let lines2 : Vec<String> = super::parse::sentences(text2);
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
            println!("    {}", lines1[ix]);
            println!("    {}", lines2[ix]);
        }
    }
}
