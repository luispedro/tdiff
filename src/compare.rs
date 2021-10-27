use std::cmp::max;

enum DiffElement {
    Equal(String),
    Insert1(String),
    Insert2(String),
    Different(String, String),
}

fn gen_diff(text1: Vec<String>, text2: Vec<String>) -> Vec<DiffElement> {
    let mut res = Vec::new();
    let max_len = max(text1.len(), text2.len());
    for ix in 0..max_len {
        if ix >= text1.len() {
            res.push(DiffElement::Insert1(text2[ix].clone()));
        } else if ix >= text2.len() {
            res.push(DiffElement::Insert2(text1[ix].clone()));
        } else if text1[ix] == text2[ix] {
            res.push(DiffElement::Equal(text1[ix].clone()));
        } else {
            res.push(DiffElement::Different(text1[ix].clone(), text2[ix].clone()));
        }
    }
    return res;
}

fn show_diff(el : &DiffElement) {
    match el {
        DiffElement::Equal(s) => {
            println!("= {}", s);
        }
        DiffElement::Insert1(s) => {
            println!("+ {}", s);
        }
        DiffElement::Insert2(s) => {
            println!("- {}", s);
        }
        DiffElement::Different(s1, s2) => {
            println!("! {}", s1);
            println!("! {}", s2);
        }
    }
}

pub fn compare(text1: String, text2: String) {
    let sentences1 : Vec<String> = super::parse::sentences(text1);
    let sentences2 : Vec<String> = super::parse::sentences(text2);
    let diff = gen_diff(sentences1, sentences2);
    for el in diff {
        show_diff(&el);
    }
}
