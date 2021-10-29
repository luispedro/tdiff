use std::cmp::{min, max};
use super::parse::*;
use colored::*;

#[derive(Debug)]

enum DiffElement<T> {
    Equal(T),
    Insert1(T),
    Insert2(T),
    Different(T, T),
}


fn compare_sentences(text1: Vec<&str>, text2: Vec<&str>) -> (u64, Vec<DiffElement<String>>) {
    if text1 == text2 {
        return (0, text1
                    .iter()
                    .map(|s| { DiffElement::Equal(s.to_string()) })
                    .collect());
    }
    let mut table : Vec<Vec<u64>> = Vec::new();
    table.resize_with(text1.len() + 1, || {
        let mut v = Vec::new();
        v.resize(text2.len() + 1, 0);
        v
    });
    for i in 0..text1.len() {
        table[i+1][0] = 1+i as u64;
    }

    for j in 0..text2.len() {
        table[0][j+1] = 1+j as u64;
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
        if p1 > 0 && p2 > 0 && val == table[p1-1][p2-1] && text1[p1-1] == text2[p2 - 1] {
            walk.push(DiffElement::Equal(text1[p1-1].to_string()));
            p1 -= 1;
            p2 -= 1;
        } else if p1 > 0 && p2 > 0 && val == table[p1-1][p2-1] + 1 && text1[p1-1] != text2[p2 - 1] {
            walk.push(DiffElement::Different(text1[p1-1].to_string(), text2[p2 - 1].to_string()));
            p1 -= 1;
            p2 -= 1;
        } else if p1 > 0 && val == table[p1-1][p2] + 1 {
            walk.push(DiffElement::Insert1(text1[p1-1].to_string()));
            p1 -= 1;
        } else {
            assert!(val == table[p1][p2-1] + 1);
            walk.push(DiffElement::Insert2(text2[p2-1].to_string()));
            p2 -= 1;
        }
    }
    walk.reverse();
    (table[text1.len()][text2.len()], walk)
}

fn print_edit_script(es : &Vec<DiffElement<String>>) {
    for el in es {
        match el {
            DiffElement::Equal(s) => print!("{} ", s),
            DiffElement::Insert1(s) => print!("{} ", s.green()),
            DiffElement::Insert2(s) => print!("{} ", s.red()),
            DiffElement::Different(s1,s2) => print!("{}{} ", s1.green(), s2.red()),
        }
    }
    println!("");
}

fn gen_diff(text1: Vec<Sentence>, text2: Vec<Sentence>) -> Vec<DiffElement<String>> {
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

fn show_diff(el : &DiffElement<String>) {
    match el {
        DiffElement::Equal(_) => {
        }
        DiffElement::Insert1(s) => {
            println!("+ {}", s.green());
        }
        DiffElement::Insert2(s) => {
            println!("- {}", s.red());
        }
        DiffElement::Different(s1, s2) => {
            print_edit_script(&compare_sentences(s1.split(" ").collect(), s2.split(" ").collect()).1);
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

#[test]
fn test_compare_sentences() {
    assert!(
        compare_sentences("Hello world".split(" ").collect(), "Hello world".split(" ").collect()).0
        == 0);
    assert!(
        compare_sentences("Hello world".split(" ").collect(), "Hello cruel world".split(" ").collect()).0
        == 1);
    assert!(
        compare_sentences("Hello world".split(" ").collect(), "Goodbye cruel world".split(" ").collect()).0
        == 2);
    assert!(
        compare_sentences("Hello world".split(" ").collect(), "Goodbye mediocre Paris".split(" ").collect()).0
        == 3);
}
