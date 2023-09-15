// Convert C to F
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let c_arg: &str = if args.len() < 2  { "" } else { &args[1] };
    let c: f32 = c_arg.parse().unwrap_or(0.0);
    print!("The temperature (Fahrenheit): {}", (9.0/5.0)*c+32.0);
}