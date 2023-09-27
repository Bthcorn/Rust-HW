use std::{
    error::Error,
    io::{Write, Read},
};
use csv::Trim;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    x: f64,
    y: f64,
    r: f64, 
}

#[derive(Debug, Clone)]
pub struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>
}

pub const PI: f64 = 3.14;
pub fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
    let mut result = Vec::new();
    for layer in layers {
        let mut total_area = 0.0;
        let mut count = 0;

        for circle in &layer.objects {
            let area = PI*(circle.r).powf(2.0);
            total_area += area;
            count += 1;
        }
        let average_area = total_area / count as f64;
        result.push((layer.name.clone(), average_area))
    }
    result
}

pub fn save_points<W: Write>(wtr: W, layers: &[(String, f64)]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        // .quote_style(QuoteStyle::Never)
        .from_writer(wtr);

        for layer in  layers {
            // let points: Vec<String> = layer.objects.iter()
            //     .map(|point| format!("{},{},{}", point.x, point.y,point.r))
            //     .collect();
            let area: String = format!("{}", layer.1);
            wtr.write_record(&[layer.0.clone(), area.to_string()])?;
        }
    wtr.flush()?;
    Ok(())
}

pub fn load_layers<R: Read>(reader: R) -> Vec<Layer> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(reader);

    let mut layers = Vec::new();

    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                // Check if there are enough fields in the record
                if record.len() >= 3 {
                    let name = record[0].trim().to_owned();
                    let color = record[1].trim().to_owned();
                    let mut objects = Vec::new();
                    // Start at index 2 because the first two columns are name and color
                    for i in 2..record.len() {
                        let values: Vec<f64> = record[i]
                            .split(',')
                            .map(|s| s.trim().parse::<f64>().unwrap_or(0.0))
                            .collect();

                        for chunk in values.chunks(3) {
                            if chunk.len() == 3 {
                                objects.push(Circle {
                                    x: chunk[0],
                                    y: chunk[1],
                                    r: chunk[2],
                                });
                            }
                        }
                    }
                    layers.push(Layer { name, color, objects });
                } else {
                    eprintln!("Error: Invalid record - not enough fields");
                }
            }
            Err(err) => {
                eprintln!("Error reading CSV: {}", err);
            }
        }
    }
    layers
}