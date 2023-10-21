use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
}

// pub enum Shape {
//     Circle(i64, i64, i64),
//     Rectangle(i64, i64, i64, i64),
//     Triangle(i64, i64, i64, i64, i64, i64)
// }

// impl Shape {
//     fn rep_string(&self) -> String {
//         match self {
//             Shape::Circle(x, y, r) => {
//                 format!("<Circle: {}, {}, {}>", x, y, r)
//             }
//             Shape::Rectangle(x, y, w, h) => {
//                 format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h)
//             }
//             Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3) => {
//                 format!("<Triangle: {}, {}, {}, {}, {}, {}>", x1, y1, x2 ,y2, x3, y3)
//             }
//         }
//     }
//     fn area(&self) -> f64 {
//         match self {
//             Shape::Circle(_, _, r) => PI * (r.to_owned() as f64).powi(2),
//             Shape::Rectangle(_, _, w, h) => w.to_owned() as f64 * h.to_owned() as f64,
//             Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3 ) =>  0.5 * ((x1.to_owned() as f64 - x3.to_owned() as f64) * (y2.to_owned() as f64 - y1.to_owned() as f64) - (x1.to_owned() as f64 - x2.to_owned() as f64) * (y3.to_owned() as f64 - y1.to_owned() as f64)),

//         }
//     }
// }

// const INPUT_SHAPES: &[Shape] = &[
//     Shape::Circle(0, 0, 1),
//     Shape::Circle(50, 50, 15),
//     Shape::Rectangle(40, 40, 20, 20),
//     Shape::Rectangle(10, 40, 15, 10),
//     Shape::Triangle(3, 4, 8, 2, 1, 7),
//     Shape::Triangle(5, 1, 9, 6, 2, 3)
// ];
// const EXPECTED: &[&str] = &[
//     "<Circle: 0, 0, 1>, area: 3.14",
//     "<Circle: 50, 50, 15>, area: 706.86",
//     "<Rectangle: 40, 40, 20, 20>, area: 400.00",
//     "<Rectangle: 10, 40, 15, 10>, area: 150.00",
//     "<Triangle: 3, 4, 8, 2, 1, 7>, area: 5.50",
//     "<Triangle: 5, 1, 9, 6, 2, 3>, area: 11.50"
// ];

// #[test]
// fn test_shapes() {
//     let input_list = INPUT_SHAPES;
//     let shape_list = input_list.clone();
//     let omap = shape_list
//         .iter()
//         .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
//     let output: Vec<_> = omap.collect();
//     assert_eq!(output, EXPECTED);
// }

// trait Shape: ShapeClone {
//     fn rep_string(&self) -> String;
//     fn area(&self) -> f64;
// }

// trait ShapeClone {
//     fn clone_box(&self) -> Box<dyn Shape>;
// }

// impl Clone for Box<dyn Shape> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

// #[derive(Clone)]
// pub struct Triangle {
//     x1: i64,
//     y1: i64,
//     x2: i64, 
//     y2: i64,
//     x3: i64,
//     y3: i64,
// }

// impl Shape for Triangle {
//     fn rep_string(&self) -> String {
//         format!("<Triangle: {}, {}, {}, {}, {}, {}>", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3)
//     }
//     fn area(&self) -> f64 {
//         0.5 * (self.x1 as f64 - self.x3 as f64) * (self.y2 as f64 - self.y1 as f64) - (self.x1 as f64 - self.x2 as f64) * (self.y3 as f64 - self.y1 as f64)
//     }
// }

// impl ShapeClone for Triangle {
//     fn clone_box(&self) -> Box<dyn Shape> {
//         Box::new(self.clone())
//     }
// }

// impl Triangle {
//     fn new(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> Box<dyn Shape> {
//         Box::new(Triangle {x1, y1, x2, y2, x3, y3})
//     }
// }
// // ------------------------------------------------------------------------------------------
// #[derive(Clone)]
// pub struct Circle {
//     x: i64,
//     y: i64,
//     r: i64,
// }

// #[derive(Clone)]
// pub struct Rectangle {
//     x: i64,
//     y: i64,
//     w: i64,
//     h: i64,
// }

// impl ShapeClone for Circle {
//     fn clone_box(&self) -> Box<dyn Shape> {
//         Box::new(self.clone())
//     }
// }

// impl Shape for Circle {
//     fn rep_string(&self) -> String {
//         format!("<Circle: {}, {}, {}>", self.x, self.y, self.r)
//     }
//     fn area(&self) -> f64 {
//         PI * (self.r.to_owned() as f64).powi(2)
//     }
// }

// impl ShapeClone for Rectangle {
//     fn clone_box(&self) -> Box<dyn Shape> {
//         Box::new(self.clone())
//     }
// }

// impl Shape for Rectangle {
//     fn rep_string(&self) -> String {
//         format!(
//             "<Rectangle: {}, {}, {}, {}>",
//             self.x, self.y, self.w, self.h
//         )
//     }
//     fn area(&self) -> f64 {
//         self.w as f64 * self.h as f64
//     }
// }

// impl Circle {
//     fn new(x: i64, y: i64, r: i64) -> Box<dyn Shape> {
//         Box::new(Circle { x, y, r })
//     }
// }

// impl Rectangle {
//     fn new(x: i64, y: i64, w: i64, h: i64) -> Box<dyn Shape> {
//         Box::new(Rectangle { x, y, w, h })
//     }
// }

// fn input_shape_list() -> Vec<Box<dyn Shape>> {
//     vec![
//         Circle::new(0, 0, 1),
//         Circle::new(50, 50, 15),
//         Rectangle::new(40, 40, 20, 20),
//         Rectangle::new(10, 40, 15, 10),
//         Triangle::new(3, 4, 8, 2, 1, 7),
//         Triangle::new(5, 1, 9, 6, 2, 3)
//     ]
// }
// const EXPECTED_001: &[&str] = &[
//     "<Circle: 0, 0, 1>",
//     "<Circle: 50, 50, 15>",
//     "<Rectangle: 40, 40, 20, 20>",
//     "<Rectangle: 10, 40, 15, 10>",
//     "<Triangle: 3, 4, 8, 2, 1, 7>",
//     "<Triangle: 5, 1, 9, 6, 2, 3>"
// ];
// const EXPECTED_002: &[&str] = &[
//     "<Circle: 0, 0, 1>, area: 3.14",
//     "<Circle: 50, 50, 15>, area: 706.86",
//     "<Rectangle: 40, 40, 20, 20>, area: 400.00",
//     "<Rectangle: 10, 40, 15, 10>, area: 150.00",
//     "<Triangle: 3, 4, 8, 2, 1, 7>, area: 13.00",
//     "<Triangle: 5, 1, 9, 6, 2, 3>, area: 15.50",
// ];

// #[test]
// fn test_shapes_001() {
//     let shape_list = input_shape_list();
//     let omap = shape_list.iter().map(|s| s.rep_string());
//     let output: Vec<_> = omap.collect();
//     assert_eq!(output, EXPECTED_001);
// }

// #[test]
// fn test_shapes_002() {
//     let shape_list = input_shape_list();
//     let omap = shape_list
//         .iter()
//         .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
//     let output: Vec<_> = omap.collect();
//     assert_eq!(output, EXPECTED_002);
// }

// #[test]
// fn test_shapes_003() {
//     let input_list = input_shape_list();
//     let shape_list = input_list.clone();
//     let omap = shape_list
//         .iter()
//         .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
//     let output: Vec<_> = omap.collect();
//     assert_eq!(output, EXPECTED_002);
// }

