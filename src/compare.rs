use std::cmp::{min, max};
use super::parse::*;

#[derive(Debug)]
enum DiffElement {
    Equal(String),
    Insert1(String),
    Insert2(String),
    Different(String, String),
}

fn compare_sentences(text1: Vec<&str>, text2: Vec<&str>) -> (u64, Vec<DiffElement>) {
    let mut table : Vec<Vec<u64>> = Vec::new();
    table.resize_with(text2.len() + 1, || {
        let mut v = Vec::new();
        v.resize(text1.len() + 1, 0);
        v
    });
    for i in 0..text1.len() {
        table[0][i+1] = i as u64;
    }

    for j in 0..text2.len() {
        table[j+1][0] = j as u64;
    }
    for i in 1..(1+text1.len()) {
        for j in 1..(1+text2.len()) {
            table[i][j] = min(min(
                            table[i-1][i] + 1,
                            table[i][j-1] + 1),
                            table[i-1][j-1] + (if text1[i-1] == text2[j-1] { 0 } else { 1 })
                            )
        }
    }
    let mut walk = Vec::new();
    let mut p1 = text1.len();
    let mut p2 = text2.len();
    while p1 > 0 || p2 > 0 {
        let val = table[p1][p2];
        if val == table[p1-1][p2-1] && text1[p1-1] == text2[p2 - 1] {
            walk.push(DiffElement::Equal(text1[p1-1].to_string()));
            p1 -= 1;
            p2 -= 1;
        } else if val == table[p1-1][p2-1] + 1 && text1[p1-1] != text2[p2 - 1] {
            walk.push(DiffElement::Different(text1[p1-1].to_string(), text2[p2 - 1].to_string()));
            p1 -= 1;
            p2 -= 1;
        } else if val == table[p1-1][p2] + 1 {
            walk.push(DiffElement::Insert1(text1[p1-1].to_string()));
            p1 -= 1;
        } else {
            assert!(val == table[p1-1][p2] + 1);
            walk.push(DiffElement::Insert2(text2[p2-1].to_string()));
            p2 -= 1;
        }
    }
    walk.reverse();
    (table[text1.len()][text2.len()], walk)
}

fn gen_diff(text1: Vec<Sentence>, text2: Vec<Sentence>) -> Vec<DiffElement> {
    let mut res = Vec::new();
    let max_len = max(text1.len(), text2.len());
    for ix in 0..max_len {
        if ix >= text1.len() {
            res.push(DiffElement::Insert1(text2[ix].0.clone()));
        } else if ix >= text2.len() {
            res.push(DiffElement::Insert2(text1[ix].0.clone()));
        } else if text1[ix] == text2[ix] {
            res.push(DiffElement::Equal(text1[ix].0.clone()));
        } else {
            res.push(DiffElement::Different(text1[ix].0.clone(), text2[ix].0.clone()));
        }
    }
    return res;
}

fn show_diff(el : &DiffElement) {
    match el {
        DiffElement::Equal(_) => {
            println!("=");
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
            println!("D:  {:?}", compare_sentences(s1.split(" ").collect(), s2.split(" ").collect()));
        }
    }
}

pub fn compare(text1: String, text2: String) {
    let sentences1 : Vec<Sentence> = super::parse::sentences(text1);
    let sentences2 : Vec<Sentence> = super::parse::sentences(text2);
    let diff = gen_diff(sentences1, sentences2);
    for el in diff {
        show_diff(&el);
    }
}
