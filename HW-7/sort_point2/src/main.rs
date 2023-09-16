fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();
    let mut point_list = Vec::new();

    for i in 1..args.len() {
        let n: f64 = match args[i].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error parsing argument: {}", args[i]);
                return;
            }
        };

        if i % 2 != 0 {
            array1.push(n);
        } else if i % 2 == 0 {
            array2.push(n);
        }
    }

    let mut a = array1.iter();
    let mut b = array2.iter();
    while let (Some(x), Some(y)) = (a.next(), b.next()) {
        point_list.push((*x, *y))
    } 
    point_list.to_vec();
    
    // println!("{:?}", point_list);
    let ascending_point = sort_point2::ascending_point(&mut point_list);
    let descending_point = sort_point2::descending_point(&mut point_list);
    println!("Ascending: {:?}", ascending_point);
    println!("Descending: {:?}", descending_point);
}
