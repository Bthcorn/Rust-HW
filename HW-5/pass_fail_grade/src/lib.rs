pub fn split_grades(v: &[&str]) -> (Vec<String>, Vec<String>) {
    let mut grade1: Vec<String> = Vec::new();
    let mut grade2: Vec<String> = Vec::new();
    let list1 = "CBAA+";
    let list2 = "FD";

    for &c in v {
        if list1.contains(c) {
            grade1.push(c.to_string())
        } else if list2.contains(c) {
            grade2.push(c.to_string())
        }
    }
    (grade1, grade2)
}

// for c in v {
//     let c_str = c;
//     if list1.iter().any(|&grade| grade == c_str) {
//         grade1.push(c)
//     } else if list2.iter().any(|&grade| grade == c_str) {
//         grade2.push(c)
//     }
// } (grade1, grade2)

#[test]
fn test_split_grades() {
    assert_eq!(split_grades(&[]), (vec![], vec![]));
    assert_eq!(
        split_grades(&["B", "F", "A+", "D", "C"]),
        (
            vec!["B".to_string(), "A+".to_string(), "C".to_string()],
            vec!["F".to_string(), "D".to_string()]
        )
    );
    assert_eq!(
        split_grades(&["A+"]),
        (
            vec!["A+".to_string()],
            vec![]
        )
    );
}

pub fn split_scores(v: &[i32]) -> (Vec<(&'static str, i32)>, Vec<(&'static str, i32)>) {
    let mut grade1: Vec<(&'static str, i32)> = Vec::new();
    let mut grade2: Vec<(&'static str, i32)> = Vec::new();
    if v.is_empty() {
        return (grade1, grade2)
    }
    for &c in v {
        if 0 <= c && c < 50 {
            grade2.push(("F", c))
        } else if c < 61 {
            grade2.push(("D", c))
        } else if c < 71 {
            grade1.push(("C", c))
        } else if c < 81 {
            grade1.push(("B", c))
        } else if c < 95 {
            grade1.push(("A", c))
        } else if c <= 100 {
            grade1.push(("A+", c))
        };
    }
    (grade1, grade2)
}

#[test]
fn test_split_scores() {
    assert_eq!(
        split_scores(&[]),
        (vec![], vec![])
    );
    assert_eq!(
        split_scores(&[75, 42, 98, 54, 63]),
        (vec![("B", 75), ("A+", 98), ("C", 63)], vec![("F", 42), ("D", 54)])
    );
    assert_eq!(
        split_scores(&[98]),
        (vec![("A+", 98)], vec![])
    );
    


}
