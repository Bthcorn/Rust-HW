pub fn make_grades (v: &mut [i64]) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new(); 
    for n in 0..v.len() {
        let x: &str = if v[n] < 0 {
            "Invalid score"
        } else if v[n] < 50 { 
            "Failed with F"
        } else if v[n] < 61 {
            "D"
        } else if v[n] < 71 {
            "C"
        } else if v[n] < 81 {
            "B"
        } else if v[n] < 95 {
            "A"
        } else if v[n] <= 100 {
            "Excellent with A+"
        } else {
            "Invalid score"
        }; result.push(x);
        
    } result
}

#[test]
fn test_make_grades() {
    let array: &mut [i64] = &mut [0, 50, 65, 78, 95, 101];
    let grades: Vec<&str> = make_grades(array);
    assert_eq!(grades, &["Failed with F", "D", "C", "B", "Excellent with A+", "Invalid score"]);
}

pub fn make_grades_re (v: &mut [i64]) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let n = 0; 
    if !v.is_empty()  {
        let x: &str = if v[n] < 0 {
            "Invalid score"
        } else if v[n] < 50 { 
            "Failed with F"
        } else if v[n] < 61 {
            "D"
        } else if v[n] < 71 {
            "C"
        } else if v[n] < 81 {
            "B"
        } else if v[n] < 95 {
            "A"
        } else if v[n] <= 100 {
            "Excellent with A+"
        } else {
            "Invalid score"
        }; 
        result.push(x);
        result.extend(make_grades_re(&mut v[1..]));
        
    } result
}

#[test]
fn test_make_grades_re() {
    let array: &mut [i64] = &mut [0, 50, 65, 78, 95, 101];
    let grades: Vec<&str> = make_grades_re(array);
    assert_eq!(grades, &["Failed with F", "D", "C", "B", "Excellent with A+", "Invalid score"]);
}
