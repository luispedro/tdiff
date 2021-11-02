use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Debug, Clone)]
pub struct Sentence {
    pub content: String,
    pub n_words: i64,
    pub hash_words: i64,
}

impl PartialEq for Sentence {
    fn eq(&self, other: &Self) -> bool {
        self.hash_words == other.hash_words && self.content == other.content
    }
}

impl Sentence {
    pub fn mk_sentence(content : String) -> Self {
        let words : Vec<&str> = content
                        .split_whitespace()
                        .collect();
        let n_words = words.len() as i64;
        let mut hash_words = 0;
        for w in words {
            let mut hasher = DefaultHasher::new();
            w.hash(&mut hasher);
            let h = hasher.finish();
            hash_words |= 1 << (h % 0x3f);
        }
        Sentence { content, n_words, hash_words }
    }
    pub fn words<'a>(self : &'a Self) -> Vec<&'a str> {
        self
            .content
            .split_whitespace()
            .collect()
    }
}

fn normalize_sentence(sen : &str) -> String {
    return sen
            .trim()
            .split_whitespace()
            .intersperse(" ")
            .collect();
}

pub fn sentences(text : String) -> Vec<Sentence> {
    return text
        .split(".")
        .map(normalize_sentence)
        .filter(|s| { s.len() > 0 })
        .map(Sentence::mk_sentence)
        .collect();
}

#[test]
fn test_trim_space() {
    assert!(
        normalize_sentence("Hello world")
        == "Hello world");
    assert!(
        normalize_sentence("   Hello world    ")
        == "Hello world");
    assert!(
        normalize_sentence("   Hello    world    ")
        == "Hello world");
    assert!(
        normalize_sentence("\tHello\n\nworld    ")
        == "Hello world");
}

#[test]
fn test_sentences() {
    assert!(
        sentences("Hello world. My name is tdiff.\n\n\n".to_string()).len()
        == 2);
}
