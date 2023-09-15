fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x_arg: &str = if args.len() < 2 {""} else {&args[1]};
    let x: i32 = match x_arg.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid score.");
            return;
        },
    };
    
    let grade: &str = if x < 0 {
        "Invalid score"
    } else if x < 50 { 
        "Failed with F"
    } else if x < 61 {
        "D"
    } else if x < 71 {
        "C"
    } else if x < 81 {
        "B"
    } else if x < 95 {
        "A"
    } else if x <= 100 {
        "Excellent with A+"
    } else {
        "Invalid score"
    };
    println!("Your grade is: {}.", grade)
}
