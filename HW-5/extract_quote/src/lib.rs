pub fn extract_quoted_words(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::<String>::new();
    if text.is_empty() {
        return result;
    }

    for t in text.split_whitespace() {
        if t.starts_with("*") && t.ends_with("*") {
            let x = t.trim_start_matches("*").trim_end_matches("*").to_string();
            result.push(x);
        }
    }
    result
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        vec!["", "C++", "Python"] // "**", "*C++*", "*Python*"
    );
    assert_eq!(extract_quoted_words_r("*Python* "), vec!["Python"]);
    assert_eq!(extract_quoted_words_r("***"), [""]);
    
}

pub fn extract_quoted_words_r(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let v: Vec<&str> = text.split_whitespace().collect();
    if text.is_empty() {
        return result;
    } else {
        if v[0].starts_with("*") && v[0].ends_with("*") {
            let x = v[0]
                .trim_start_matches("*")
                .trim_end_matches("*")
                .to_string();
            result.push(format!("{}", x));
        }
    }
    result.extend(extract_quoted_words_r(&v[1..].join(" ")));
    result
}

#[test]
fn test_extract_quoted_words_r() {
    assert_eq!(extract_quoted_words_r(""), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words_r("C ** *C++* *Java *Python* Rust*"),
        vec!["", "C++", "Python"] // "**", "*C++*", "*Python*"
    );
    assert_eq!(extract_quoted_words_r("*Python* "), vec!["Python"]);
    assert_eq!(extract_quoted_words_r("C ** "), [""]);
}
