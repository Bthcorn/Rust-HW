pub fn count_vowels(text: &str) -> usize {
    let mut count = 0;
    let vowels = "aeiouAEIOU";

    for c in text.chars() {
        if vowels.contains(c) {
            count += 1
        }
    } count
}

#[test]
fn test_vowels_count() {
assert_eq!(count_vowels(""), 0);
assert_eq!(count_vowels("abEcd"), 2);
assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
}

pub fn count_vowels_r(text: &str) -> usize {
    let vowels = "aeiouAEIOU";
    if text.is_empty() {
        return 0;
    } 
    let first_t = text.chars().next().unwrap(); 
    let remaining_t =  &text[1..];
    let mut count = if vowels.contains(first_t) {
        1
    } else {
        0
    }; count += count_vowels_r(remaining_t);
    count
}

#[test]
fn test_vowels_count_r() {
assert_eq!(count_vowels_r(""), 0);
assert_eq!(count_vowels_r("abEcd"), 2);
assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
}

pub fn count_vowels_v2(text: &str) -> Vec<(String, usize)> {
    let vowels = "aeiouAEIOU";
    let mut result: Vec<(String, usize)> = Vec::new();
    for half in text.split_whitespace() {
        let mut count: usize = 0;
        for c in half.chars() {
            if vowels.contains(c) {
                count += 1;
            }
        } result.push((half.to_string(), count))
    } result
}
// add more cases to test!!!
#[test]
fn test_vowels_count_v2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
        ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
        ("7x8U3y5z".to_string(), 1) // 'U'
        ]
    );
}
