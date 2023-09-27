use clap::{Arg, App};
use std::{
    error::Error,
    fs::File,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = 
            App::new("Read and write area of circle calculation.")
                .version("0.1.0")
                .about("To read file and save the area calculation to CSV file.")
                .arg(
                    Arg::with_name("Input_file")
                        .value_name("FILE")
                        .short("i")
                        .takes_value(true)
                        .help("Input file name")
                )
                .arg(
                    Arg::with_name("Output_file")
                        .value_name("FILE")
                        .short("o")
                        .takes_value(true)
                        .help("Output file name")
                )
                .get_matches();

            let input = match matches.value_of("Input_file") {
                Some(val) => val,
                None => {
                    eprintln!("Error: Missing Input_file argument");
                    std::process::exit(1);
                }
            };
        
            // Attempt to get the "Output_file" value from matches
            let output = match matches.value_of("Output_file") {
                Some(val) => val,
                None => {
                    eprintln!("Error: Missing Output_file argument");
                    std::process::exit(1);
                }
            };

        let in_path = File::open(input)?;
        let out_path = File::create(output)?;
        let layers = gen_cal_area::load_layers(in_path);
        let area = gen_cal_area::cal_average_area(&layers);
        let x = gen_cal_area::save_points(out_path,&area);
        Ok(())
}
