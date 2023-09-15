// convert F to C
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f_arg: &str = if args.len() < 2 { "" } else { &args[1]};
    let f: f32 = f_arg.parse().unwrap_or(0.0);
    print!("The temperature (Celsius): {:.4}", (5.0/9.0)*(f-32.0));
}
