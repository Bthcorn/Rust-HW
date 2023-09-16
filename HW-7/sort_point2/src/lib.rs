pub fn ascending_point(v: &mut Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    for _i in 0..v.len() {
        for j in 0..v.len() - 1 {
            if v[j].0 > v[j + 1].0 {
                v.swap(j, j + 1);
            } else if v[j].0 == v[j + 1].0 && v[j].1 > v[j + 1].1 {
                v.swap(j, j + 1);
            }
        }
    }
    v.to_vec()
}

pub fn descending_point(v: &mut Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    for _i in 0..v.len() {
        for j in 0..v.len() - 1 {
            if v[j].0 < v[j + 1].0 {
                v.swap(j, j + 1);
            } else if v[j].0 == v[j + 1].0 && v[j].1 < v[j + 1].1 {
                v.swap(j, j + 1);
            }
        }
    }
    v.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_sort_point() {
        let test = ascending_point(&mut vec![(2.0, 3.0), (1.0, 4.0), (3.0, 2.0), (2.0, 1.0)]);
        assert_eq!(test, [(1.0, 4.0), (2.0, 1.0), (2.0, 3.0), (3.0, 2.0)]);
    }

    #[test]
    fn test_descending_sort_point() {
        let test = descending_point(&mut vec![(2.0, 3.0), (1.0, 4.0), (3.0, 2.0), (2.0, 1.0)]);
        assert_eq!(test, [(3.0, 2.0), (2.0, 3.0), (2.0, 1.0), (1.0, 4.0)]);
    }
}