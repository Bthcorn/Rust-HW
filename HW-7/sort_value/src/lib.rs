pub fn ascending_sort(v: &mut Vec<i64>) -> Vec<i64> {
    v.sort_by(|a, b| a.cmp(b));
    v.to_vec()
}

pub fn descending_sort(v: &mut Vec<i64>) -> Vec<i64> {
    v.sort_by(|a, b| b.cmp(a));
    v.to_vec()
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