use std::cmp::min;
use super::parse::*;
use colored::*;

#[derive(Debug)]
enum DiffElement<T> {
    Equal(T),
    Insert1(T),
    Insert2(T),
    Different(T, T),
}


fn edit_distance<T: PartialEq + Clone>(text1: &Vec<T>, text2: &Vec<T>, cmp: fn(&T, &T) -> u64, icost: u64) -> (u64, Vec<DiffElement<T>>) {
    let mut table : Vec<Vec<u64>> = Vec::new();
    table.resize_with(text1.len() + 1, || {
        let mut v = Vec::new();
        v.resize(text2.len() + 1, 0);
        v
    });
    for i in 0..text1.len() {
        table[i+1][0] = icost * (1+i) as u64;
    }

    for j in 0..text2.len() {
        table[0][j+1] = icost * (1+j) as u64;
    }
    for i in 1..(1+text1.len()) {
        for j in 1..(1+text2.len()) {
            table[i][j] = min(min(
                            table[i-1][j] + icost,
                            table[i][j-1] + icost),
                            table[i-1][j-1] + cmp(&text1[i-1], &text2[j-1])
                            )
        }
    }
    let mut walk = Vec::new();
    let mut p1 = text1.len();
    let mut p2 = text2.len();
    while p1 > 0 || p2 > 0 {
        let val = table[p1][p2];
        if p1 > 0 && p2 > 0 && val == table[p1-1][p2-1] && text1[p1-1] == text2[p2 - 1] {
            walk.push(DiffElement::Equal(text1[p1-1].clone()));
            p1 -= 1;
            p2 -= 1;
        } else if p1 > 0 && p2 > 0 && val == table[p1-1][p2-1] + cmp(&text1[p1-1], &text2[p2 - 1]) {
            walk.push(DiffElement::Different(text1[p1-1].clone(), text2[p2 - 1].clone()));
            p1 -= 1;
            p2 -= 1;
        } else if p1 > 0 && val == table[p1-1][p2] + icost {
            walk.push(DiffElement::Insert1(text1[p1-1].clone()));
            p1 -= 1;
        } else {
            assert!(val == table[p1][p2-1] + icost);
            walk.push(DiffElement::Insert2(text2[p2-1].clone()));
            p2 -= 1;
        }
    }
    walk.reverse();
    (table[text1.len()][text2.len()], walk)
}

fn too_different(text1: &Sentence, text2: &Sentence) -> bool {
    let n1 = text1.hash_words.count_ones();
    let n2 = text2.hash_words.count_ones();
    let common = (text1.hash_words & text2.hash_words).count_ones();
    assert!(min(n1, n2) >= common);
    return min(n1,n2) - common > min(n1,n2)/2;
}

fn compare_sentences<'a>(text1: &'a Sentence, text2: &'a Sentence) -> (u64, Vec<DiffElement<&'a str>>) {
    if text1 == text2 {
        return (0, text1
                    .content
                    .split_whitespace()
                    .map(|s| { DiffElement::Equal(s) })
                    .collect());
    } else if too_different(&text1, &text2) {
        return ((text1.n_words + text2.n_words) as u64, vec![DiffElement::Different(&text1.content, &text2.content)]);

    }
    edit_distance(
                 &text1.words(),
                 &text2.words(),
                 |t1, t2| { if t1 == t2 { 0 } else { 1 } },
                 1)
}


fn print_edit_script(es : &Vec<DiffElement<&str>>) {
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

pub fn compare(text1: String, text2: String) {
    let sentences1 : Vec<Sentence> = super::parse::sentences(text1);
    let sentences2 : Vec<Sentence> = super::parse::sentences(text2);
    let diff = edit_distance(
                    &sentences1,
                    &sentences2,
                    |t1,t2| { compare_sentences(&t1, &t2).0 },
                    4).1;
    for el in diff {
        match el {
            DiffElement::Equal(_) => {
            }
            DiffElement::Insert1(s) => {
                println!("+ {}", s.content.green());
            }
            DiffElement::Insert2(s) => {
                println!("- {}", s.content.red());
            }
            DiffElement::Different(s1, s2) => {
                print_edit_script(&compare_sentences(&s1, &s2).1);
            }
        }
    }
}

#[test]
fn test_compare_sentences() {
    let to_s = |s:&str| { Sentence::mk_sentence(s.to_string()) };
    assert!(
        compare_sentences(&to_s("Hello world"), &to_s("Hello world")).0
        == 0);
    assert!(
        compare_sentences(&to_s("Hello world"), &to_s("Hello cruel world")).0
        == 1);
    assert!(
        compare_sentences(&to_s("Hello world"), &to_s("Goodbye cruel world")).0
        == 2);
    assert!(
        compare_sentences(&to_s("Hello world"), &to_s("Goodbye mediocre Paris")).0
        == 3);
}
