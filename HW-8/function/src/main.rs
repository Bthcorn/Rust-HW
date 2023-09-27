fn main() {
    let v = vec![
        function::Point {
            x: 1.0,
            y: 2.0,
        },
        function::Point {
            x: 3.0,
            y: 4.0,
        },
    ];
    let v1 = function::to_polar(&v);
    let v2 = function::to_cartesian(&v1);
    println!("{:?}", v1);
    println!("{:?}", v2)
}
