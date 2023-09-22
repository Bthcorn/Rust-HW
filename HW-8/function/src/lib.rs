#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct PolarPoint {
    pub r: f64,
    pub t: f64,
}


pub fn to_polar(v: &[Point]) -> Vec<PolarPoint> {
    let mut result = Vec::new();
    for point in v {
        let r1 = (point.x.powf(2.) + point.y.powf(2.)).sqrt();
        let t1 = (point.y / point.x).atan();
        result.push(PolarPoint {r: r1, t: t1});
    } result
}


pub fn to_cartesian(v: &[PolarPoint]) -> Vec<Point> {
    let mut result = Vec::new();
    for point in v {
        let x1 = (point.r)*(point.t.cos());
        let y1 = (point.r)*(point.t.sin());
        result.push(Point {x :x1, y: y1});
    } result
}



