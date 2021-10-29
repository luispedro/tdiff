#[derive(Clone, PartialEq)]
pub struct Sentence(pub String);

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
        .map(Sentence)
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
