fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 4 {
        let f1: i32 = args[1].parse().unwrap_or(0);
        let f2: i32 = args[2].parse().unwrap_or(0);
        let d: i32 = args[3].parse().unwrap_or(0);
        let n: i32 = (f2 + f1) / d;

        let mut f1: i32 = f1;
        let d: i32 = (f2 - f1) / n;
        println!("  Fahr\tCelcius");

        let total_lines = n + 1;

        for _i in 0..total_lines {
            let c: f32 = temptable::temp(f1 as f32);
            println!("{:>6}\t{:>6.1}", f1, c);
            f1 += d;
            }
           
        }
     else {
        println!("Please provide exactly four command-line arguments: f1: 1st Fahr, f2: last Fahr, d: difference.")
    }
}

