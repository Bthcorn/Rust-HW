use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
}

pub enum Shape {
    Circle(i64, i64, i64),
    Rectangle(i64, i64, i64, i64),
    Triangle(i64, i64, i64, i64, i64, i64)
}

impl Shape {
    fn rep_string(&self) -> String {
        match self {
            Shape::Circle(x, y, r) => {
                format!("<Circle: {}, {}, {}>", x, y, r)
            }
            Shape::Rectangle(x, y, w, h) => {
                format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h)
            }
            Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3) => {
                format!("<Triangle: {}, {}, {}, {}, {}, {}>", x1, y1, x2 ,y2, x3, y3)
            }
        }
    }
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(_, _, r) => PI * (r.to_owned() as f64).powi(2),
            Shape::Rectangle(_, _, w, h) => w.to_owned() as f64 * h.to_owned() as f64,
            Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3 ) =>  0.5 * ((x1.to_owned() as f64 - x3.to_owned() as f64) * (y2.to_owned() as f64 - y1.to_owned() as f64) - (x1.to_owned() as f64 - x2.to_owned() as f64) * (y3.to_owned() as f64 - y1.to_owned() as f64)),

        }
    }
}

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0, 0, 1),
    Shape::Circle(50, 50, 15),
    Shape::Rectangle(40, 40, 20, 20),
    Shape::Rectangle(10, 40, 15, 10),
    Shape::Triangle(3, 4, 8, 2, 1, 7),
    Shape::Triangle(5, 1, 9, 6, 2, 3)
];
const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: 3, 4, 8, 2, 1, 7>, area: 5.50",
    "<Triangle: 5, 1, 9, 6, 2, 3>, area: 11.50"
];

#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list
        .iter()
        .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}