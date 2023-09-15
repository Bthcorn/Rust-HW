use float_cmp::approx_eq;

#[test]
fn test_temp_0_300_20() {
    let expected_result = [
        (0.0, -17.8),
        (20.0, -6.7),
        (40.0, 4.4),
        (60.0, 15.6),
        (80.0, 26.7),
        (100.0, 37.8),
        (120.0, 48.9),
        (140.0, 60.0),
        (160.0, 71.1),
        (180.0, 82.2),
        (200.0, 93.3),
        (220.0, 104.4),
        (240.0, 115.6),
        (260.0, 126.7),
        (280.0, 137.8),
        (300.0, 148.9),
    ];

    for (input, expected) in &expected_result {
        assert!(approx_eq!(f32, temptable::temp(*input) , *expected, epsilon = 0.1));
    }
}

#[test]
fn test_temp_0_280_40() {
    let expected_result = [
        (0.0, -17.8),
        (40.0, 4.4),
        (80.0, 26.7),
        (80.0, 26.7),
        (120.0, 48.9),
        (200.0, 93.3),
        (240.0, 115.6),
        (280.0, 137.8),
    ];

    for (input, expected) in &expected_result {
        assert!(approx_eq!(f32, temptable::temp(*input) , *expected, epsilon = 0.1));
    }
}

#[test]
fn test_temp_300_0_20() {
    let expected_result = [
        (0.0, -17.8),
        (20.0, -6.7),
        (40.0, 4.4),
        (60.0, 15.6),
        (80.0, 26.7),
        (100.0, 37.8),
        (120.0, 48.9),
        (140.0, 60.0),
        (160.0, 71.1),
        (180.0, 82.2),
        (200.0, 93.3),
        (220.0, 104.4),
        (240.0, 115.6),
        (260.0, 126.7),
        (280.0, 137.8),
        (300.0, 148.9),
        
    ];

    for (input, expected) in &expected_result {
        assert!(approx_eq!(f32, temptable::temp(*input) , *expected, epsilon = 0.1));
    }
}
