fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_args: &str = if args.len() < 2 {""} else {&args[1]};
    let n: i32 = if !n_args.is_empty() {
        match n_args.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please input a valid number.");
                return;
            }
        }
    } else {
        0
    };

    let total_lines = 2*n-1;
    
    for _i in 0..total_lines {
        if _i < n{
            for _j in 0.._i+1 {
            print!("*");
            }
        } else {
            let num_asterisks = total_lines - _i;
            for _j in 0..num_asterisks {
                print!("*");
                }
        } println!(); 
    }
}
