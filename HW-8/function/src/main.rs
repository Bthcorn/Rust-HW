fn main() {
    let v = vec![
        to_polar::Point {
            x: 1.0,
            y: 2.0,
        },
        to_polar::Point {
            x: 3.0,
            y: 4.0,
        },
    ];
    let v1 = to_polar::to_polar(&v);
    let v2 = to_polar::to_cartesian(&v1);
    println!("{:?}", v1);
    println!("{:?}", v2)
}
