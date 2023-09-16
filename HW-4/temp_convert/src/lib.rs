pub fn fahr_to_cel_v (v: &mut[f32]) {
    for n in 0..v.len() {
        v[n] = ((5.0 / 9.0) * (v[n] - 32.0)*10.0).round() /10.0;
    }
}

pub fn fahr_to_cel_v_re (v: &mut[f32]) -> Vec<f32> {
    if !v.is_empty() {
        v[0] = ((5.0 / 9.0) * (v[0] - 32.0)*10.0).round() /10.0;
        return fahr_to_cel_v_re(&mut v[1..]);
    };
    v.to_vec()
}

#[test]
fn test_fahr_to_cel_v () {
    let array: &mut [f32] = &mut [0.0, 20.0, 40.0, 60.0, 80.0, 100.0];
    fahr_to_cel_v(array);
    assert_eq!(array, &[-17.8, -6.7, 4.4, 15.6, 26.7, 37.8]);
} 

#[test]
fn test_fahr_to_cel_v_re () {
    let array: &mut [f32] = &mut [0.0, 20.0, 40.0, 60.0, 80.0, 100.0];
    fahr_to_cel_v_re(array);
    assert_eq!(array, &[-17.8, -6.7, 4.4, 15.6, 26.7, 37.8]);
}