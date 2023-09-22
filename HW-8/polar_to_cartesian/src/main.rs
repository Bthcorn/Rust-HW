use std::{
    error::Error,
    fs::File,
    io::{self, Write, Read},
};

// Define the Point and PolarPoint structs
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

/// Converts a list of Cartesian points to Polar coordinates.
fn to_polar(points: Vec<Point>) -> Vec<PolarPoint> {
    points
        .iter()
        .map(|point| {
            let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
            let t = point.y.atan2(point.x);
            PolarPoint { r, t }
        })
        .collect()
}

/// Converts a list of Polar coordinates to Cartesian points.
fn to_cartesian(polar_points: Vec<PolarPoint>) -> Vec<Point> {
    polar_points
        .iter()
        .map(|polar_point| {
            let x = polar_point.r * polar_point.t.cos();
            let y = polar_point.r * polar_point.t.sin();
            Point { x, y }
        })
        .collect()
}

pub fn load_points<R: Read>(rdr: R) -> Vec<PolarPoint> {
    // let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(rdr);
    
    let mut points = vec![];

    for result in rdr.records() {
        if let Ok(record) = result {
            let x1: f64 = record[0].parse().unwrap();
            let y1: f64 = record[1].parse().unwrap();
            // let tag: String = record[2].parse()?;
            points.push( PolarPoint{r: x1, t: y1});
        }
    }
    points
}

pub fn save_points<W: Write>(wtr: W, pt_list: Vec<Point>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(wtr);

    for point in pt_list {
        wtr.write_record(&[(point.x).to_string(), (point.y).to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Input and output file paths
    let input_file = "input.csv";
    let output_file = "output.csv";

    // Load Cartesian points from input CSV
    let file = File::open(input_file)?;
    let points = load_points(file);
    println!("{:?}", points);
    
    // Convert to Polar coordinates
    let cartesian_points = to_cartesian(points);
    println!("{:?}", cartesian_points);
    // Save Polar coordinates to output CSV
    let writer = File::create(output_file)?;
    save_points(writer, cartesian_points)?;

    println!("Conversion completed successfully.");
    Ok(())
}
