use clap::{App, Arg};
use std::{
    error::Error,
    fs::File, 
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = 
    App::new("Generate a list of object layers")
            .version("0.1.0")
            .author("Anonymous")
            .about("To save a list into CSV file")
            .arg(
                Arg::with_name("layers")
                    .value_name("Layers")
                    .short("n")
                    .takes_value(true)
                    .help("Number of layers")
            )
            .arg(
                Arg::with_name("output file")
                    .value_name("FILE")
                    .short("o")
                    .takes_value(true)
                    .help("Output file name (.csv)")
            )
            .get_matches();

        let n_arg = match matches.value_of("layers") {
            Some(val) => val,
            None => {
                eprintln!("Error: Missing n number of layers");
                std::process::exit(1);
            }
        };
        let n: i32 = n_arg.parse().expect("expect n number of layer");
        
        let output_name = match matches.value_of("Output_file") {
            Some(val) => val,
            None => {
                eprintln!("Error: Missing Output_file argument");
                std::process::exit(1);
            }
        };

        let writer = File::create(output_name)?;
        let mut rng = rand::thread_rng();
        let layer_list = gen_list_layer::gen_obj_layer_list(&mut rng, n);
        let _ = gen_list_layer::save_points(writer, &layer_list)?;
        Ok(())
}
