pub fn make_arrows1 (v: &mut [i64]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for n in 0..v.len() {
        let total_lines = 2*v[n]-1;
        let mut arrow_str:String = String::new();
        for _i in 0..total_lines {
            if _i < v[n] {
                for _j in 0.._i+1 {
                arrow_str.push('*');
                }
            } else {
                let num_asterisks = total_lines - _i;
                for _j in 0..num_asterisks {
                    arrow_str.push('*');
                    }
            } arrow_str.push('\n'); 
        } result.push(arrow_str)
    } result
}

#[test]
fn test_make_arrow1 () {
    let array: &mut [i64] = &mut [0, 1, 2, 3, 4, 5];
    let arrows : Vec<String> = make_arrows1(array);
    assert_eq!(arrows, &["", "*\n", "*\n**\n*\n", 
    "*\n**\n***\n**\n*\n", 
    "*\n**\n***\n****\n***\n**\n*\n", 
    "*\n**\n***\n****\n*****\n****\n***\n**\n*\n"]);
}

pub fn make_arrows1_re (v: &mut [i64]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if !v.is_empty() {
        let total_lines = 2*v[0]-1;
        let mut arrow_str:String = String::new();
        for _i in 0..total_lines {
            if _i < v[0] {
                for _j in 0.._i+1 {
                arrow_str.push('*');
                }
            } else {
                let num_asterisks = total_lines - _i;
                for _j in 0..num_asterisks {
                    arrow_str.push('*');
                    }
            } arrow_str.push('\n'); 
        } 
        result.push(arrow_str);
        result.extend(make_arrows1_re(&mut v[1..]));
    } result
}

#[test]
fn test_make_arrow1_re () {
    let array: &mut [i64] = &mut [0, 1, 2, 3, 4, 5];
    let arrows : Vec<String> = make_arrows1_re(array);
    assert_eq!(arrows, &["", "*\n", "*\n**\n*\n", 
    "*\n**\n***\n**\n*\n", 
    "*\n**\n***\n****\n***\n**\n*\n", 
    "*\n**\n***\n****\n*****\n****\n***\n**\n*\n"]);
}

pub fn make_arrow2 (v: &mut [i64]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for n in 0..v.len() {
        let total_lines = 2*v[n]-1;
        let mut arrow_str:String = String::new();
        for _i in 0..total_lines {
            if _i < v[n] {
                for _j in 0..v[n]-(_i+1) {
                    arrow_str.push(' ');
                }
                for _j in 0.._i+1 {
                    arrow_str.push('*');
                }
            } else {
                let num_asterisks = total_lines - _i;
                for _j in 0.._i-v[n]+1 {
                    arrow_str.push(' ');
                    }
                for _j in 0..num_asterisks {
                    arrow_str.push('*');
                }
            } arrow_str.push('\n');
        } result.push(arrow_str)
    } result
}

#[test]
fn test_make_arrow2 () {
    let array: &mut [i64] = &mut [0, 1, 2, 3, 4, 5];
    let arrows : Vec<String> = make_arrow2(array);
    assert_eq!(arrows, &["", "*\n", " *\n**\n *\n", 
    "  *\n **\n***\n **\n  *\n", 
    "   *\n  **\n ***\n****\n ***\n  **\n   *\n", 
    "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n"]);
}
pub fn make_arrow2_re (v: &mut [i64]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if !v.is_empty() {
        let total_lines = 2*v[0]-1;
        let mut arrow_str:String = String::new();
        for _i in 0..total_lines {
            if _i < v[0] {
                for _j in 0..v[0]-(_i+1) {
                    arrow_str.push(' ');
                }
                for _j in 0.._i+1 {
                    arrow_str.push('*');
                }
            } else {
                let num_asterisks = total_lines - _i;
                for _j in 0.._i-v[0]+1 {
                    arrow_str.push(' ');
                    }
                for _j in 0..num_asterisks {
                    arrow_str.push('*');
                }
            } arrow_str.push('\n');
        } 
        result.push(arrow_str);
        result.extend(make_arrow2_re(&mut v[1..]))
    } result
}

#[test]
fn test_make_arrow2_re () {
    let array: &mut [i64] = &mut [0, 1, 2, 3, 4, 5];
    let arrows : Vec<String> = make_arrow2_re(array);
    assert_eq!(arrows, &["", "*\n", " *\n**\n *\n", 
    "  *\n **\n***\n **\n  *\n", 
    "   *\n  **\n ***\n****\n ***\n  **\n   *\n", 
    "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n"]);
}
