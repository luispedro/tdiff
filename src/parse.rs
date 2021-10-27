fn normalize_sentence(sen : &str) -> String {
    return sen
            .trim()
            .split_whitespace()
            .intersperse(" ")
            .collect();
}

pub fn sentences(text : String) -> Vec<String> {
    return text
        .split(".")
        .map(normalize_sentence)
        .collect();
}
