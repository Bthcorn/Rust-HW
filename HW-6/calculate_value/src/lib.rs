fn min_max_avg(v: &[i32]) -> (i32, i32, f32) {
    let mut sum = 0;
    let mut iter = v.iter();
    if let Some(mut max) = iter.next() {
        sum += max;
        let mut min = max;
        while let Some(x) = iter.next() {
            sum += x;
            if x < min {
                min = x;
            } else if x > max {
                max = x;
            }

        } let sum: i32 = v.iter().sum();
        let avr = sum as f32 / v.len() as f32;
        (*min, *max, avr)
    } else {
        (0, 0, 0.0)
    }
}

#[test]
fn test_min_max_avg() {
    assert_eq!(min_max_avg(&[]), (0, 0, 0.0));
    assert_eq!(min_max_avg(&[2, 4, 6, 7, 10]), (2, 10, 5.8));
    assert_eq!(min_max_avg(&[-1, 0, 5, -9]), (-9, 5, -1.25))
}

fn cal_partial_sums(v: &[i32]) -> Vec<i32> {
    let mut result  = Vec::new();
    let mut iter = v.iter();
    let mut num = 0;
    if let Some(x) = iter.next() {
        num += x;
        result.push(num);
        while let Some(y) = iter.next() {
            num += y;
            result.push(num)
        } result
    } else {
        return result;
    }
}

#[test]
fn test_cal_partial_sums() {
    assert_eq!(cal_partial_sums(&[]), []);
    assert_eq!(cal_partial_sums(&[2, 11, 3, 4, 7]), [2, 13, 16, 20, 27]);
    assert_eq!(cal_partial_sums(&[-4, 5, 7, -10, 7]), [-4, 1, 8, -2, 5]);
    assert_eq!(cal_partial_sums(&[-4, 5, 7]), [-4, 1, 8]);
}