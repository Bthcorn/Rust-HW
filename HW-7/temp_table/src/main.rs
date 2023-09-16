fn temp(f1: f32) -> f32 {
    (5.0 / 9.0) * (f1 - 32.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 4 {
        let f1: i32 = args[1].parse().unwrap_or(0);
        let f2: i32 = args[2].parse().unwrap_or(0);
        let d: i32 = args[3].parse().unwrap_or(0);
        let n: i32 = (f2 + f1) / d;

        let mut f1: i32 = f1;
        let d: i32 = (f2 - f1) / n;

        let total_lines = n + 1;
        println!("<style>\ntable, td, th {{\n   border: 1px solid #000000;\n   border-collapse: collapse;\n}}\n</style>");
        println!("<table>");
        println!("   <tr>\n      <th>Fahr</th>\n      <th>Cels</th>\n   </tr>");
        for _i in 0..total_lines {
            let c: f32 = temp(f1 as f32);
            gen_html(f1, c);
            f1 += d;
            }
        println!("</table>");
        } else {
      println!("Please provide exactly four command-line arguments: f1: 1st Fahr, f2: last Fahr, d: difference.")
    }
}

fn gen_html(f :i32, c: f32, ) {
    println!("   <tr>");
    println!("      <td>{:.1}</td>", f);
    println!("      <td>{:.1}</td>", c);
    println!("   </tr>")
}