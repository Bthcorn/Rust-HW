fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut array = Vec::new();

    for i in 1..args.len() {
        let n: i64 = match args[i].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error parsing argument: {}", args[i]);
                return;
            }
        };
        array.push(n);
    }

    let ascending: Vec<i64> =  sort_value2::ascending_sort(&mut array);
    let descending: Vec<i64> = sort_value2::descending_sort(&mut array);
    println!("Ascending: {:?}", ascending);
    println!("Descending: {:?}", descending);

}
