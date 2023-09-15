// Calculate area of the circle
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let r_arg = if args.len() <2 { "" } else { &args[1] };
    let r: f32 = r_arg.parse().unwrap_or(0.0);
    print!("area of the circle: {}", 3.1416*r*r);
}
