pub fn ascending_sort(v: &mut Vec<i64>) -> Vec<i64> {
    for _i in 0..v.len() { 
        for j in 0..v.len() - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j+1);
            }
        }
    } v.to_vec()
}

pub fn descending_sort(v: &mut Vec<i64>) -> Vec<i64> {
    for _i in 0..v.len() { 
        for j in 0..v.len() - 1 {
            if v[j] < v[j + 1] {
                v.swap(j, j+1);
            }
        }
    } v.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_sort() {
        let mut array = vec![5, 2, 1, 3, 4];
        let sorted = ascending_sort(&mut array);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_descending_sort() {
        let mut array = vec![5, 2, 1, 3, 4];
        let sorted = descending_sort(&mut array);
        assert_eq!(sorted, vec![5, 4, 3, 2, 1]);
    }
}