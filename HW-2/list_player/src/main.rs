fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Player 1: {}", "N/A");
        println!("Player 2: {}", "N/A");
    } else if args.len() < 3 {
        println!("Player 1: {}", &args[1]);
        println!("Player 2: {}", "N/A");
    } else {
        println!("Player 1: {}", &args[1]);
        println!("Player 2: {}", &args[2]);
    }
}


