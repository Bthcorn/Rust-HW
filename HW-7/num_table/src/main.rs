fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 4 {
        let n1: i32 = args[1].parse().unwrap_or(0);
        let n2: i32 = args[2].parse().unwrap_or(0);
        let d: i32 = args[3].parse().unwrap_or(0);
        let n: i32 = (n2 + n1) / d;

        let mut n1: i32 = n1;
        let d: i32 = (n2 - n1) / n;

        let total_lines = n + 1;
        println!("<style>\ntable, td, th {{\n   border: 1px solid #000000;\n   border-collapse: collapse;\n}}\n</style>");
        println!("<table>");
        println!("   <tr>\n      <th>x</th>\n      <th>x^2</th>\n      <th>x^3</th>\n   </tr>");
        for _i in 0..total_lines {
            let x1 = n1.pow(2);
            let x2 = n1.pow(3);
            gen_html(n1, x1, x2);
            n1 += d;
            }
        println!("</table>");
        } else {
      println!("Please provide exactly four command-line arguments: f1: 1st Fahr, f2: last Fahr, d: difference.")
    }
}

fn gen_html(x: i32, x1:i32, x2:i32) {
    println!("   <tr>");
    println!("      <td>{}</td>", x);
    println!("      <td>{}</td>", x1);
    println!("      <td>{}</td>", x2);
    println!("   </tr>")
}
