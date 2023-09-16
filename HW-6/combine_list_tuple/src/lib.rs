fn pack_number_tuples3(v1: &[i32], v2: &[i32], v3: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    let size = vec![v1.len(), v2.len(), v3.len()];
    let mut max = 0;

    for j in size {
        if j > max {
            max = j;
        }
    }

    if v1.is_empty() && v2.is_empty() && v3.is_empty() {
        return result;
    } else {
        for i in 0..max {
            let x_v1 = if i < v1.len() { v1[i] } else { 0 };
            let x_v2 = if i < v2.len() { v2[i] } else { 0 };
            let x_v3 = if i < v3.len() { v3[i] } else { 0 };
            result.push((x_v1, x_v2, x_v3));
        }
    }
    result
}

#[test]
fn test_pack_number_tuples3() {
    assert_eq!(pack_number_tuples3(&[], &[], &[]), []);
    assert_eq!(
        pack_number_tuples3(&[1, 2], &[4, 3], &[5]),
        [(1, 4, 5), (2, 3, 0)]
    );
    assert_eq!(
        pack_number_tuples3(&[2], &[4, 3], &[5]),
        [(2, 4, 5), (0, 3, 0)]
    );
    assert_eq!(
        pack_number_tuples3(&[2, 4, 5], &[4, 3], &[5]),
        [(2, 4, 5), (4, 3, 0), (5, 0, 0)]
    );
    assert_eq!(
        pack_number_tuples3(&[1], &[4, 3, 1], &[2]),
        [(1, 4, 2), (0, 3, 0), (0, 1, 0)]
    );
    assert_eq!(
        pack_number_tuples3(&[1], &[4, 3, 1, 5], &[2]),
        [(1, 4, 2), (0, 3, 0), (0, 1, 0), (0, 5, 0)]
    );
}

fn pack_number_tuples_s3(v1: &[i32], v2: &[i32], v3: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    let mut a = v1.iter();
    let mut b = v2.iter();
    let mut c = v3.iter();

    while let (Some(x1), Some(x2), Some(x3)) = (a.next(), b.next(), c.next()) {
        result.push((*x1, *x2, *x3))
    }
    result
}

#[test]
fn test_pack_number_tuples_s3() {
    assert_eq!(pack_number_tuples_s3(&[], &[], &[]), []);
    assert_eq!(pack_number_tuples_s3(&[1, 2], &[4, 3], &[5]), [(1, 4, 5)]);
    assert_eq!(
        pack_number_tuples_s3(&[1, 2], &[4, 3], &[5, 4]),
        [(1, 4, 5), (2, 3, 4)]
    );
    assert_eq!(
        pack_number_tuples_s3(&[1, 2, 4], &[4, 3], &[5, 4]),
        [(1, 4, 5), (2, 3, 4)]
    );
}
