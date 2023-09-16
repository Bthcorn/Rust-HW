fn unpack_number_tuples(v: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut result1 = Vec::new();
    let mut result2 = Vec::new();

    for (x, y) in v.iter() {
        result1.push(*x);
        result2.push(*y);

    } (result1, result2)
}

#[test]
fn test_unpack_number_tuples() {
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (3, 2), (2,1)]), (vec![1, 3, 2], vec![4, 2, 1]));
}

fn unpack_number_tuples3(v: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut result1 = Vec::new();
    let mut result2 = Vec::new();
    let mut result3 = Vec::new();

    for (x, y, z) in v.iter() {
        result1.push(*x);
        result2.push(*y);
        result3.push(*z);
    } (result1 , result2, result3)
}

#[test]
fn test_unpack_number_tuples3() {
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1)]), (vec![1, 2], vec![4, 2], vec![5, 1]));
}