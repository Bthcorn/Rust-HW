use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Please input at least one file name.");
        return;
    }
    let files: Vec<String> = args.into_iter().skip(1).collect();

    let result = count_words::read_to_doc_c(&files);
    let html = count_words::generate_html(&result);
    let mut file_path = File::create("output.html").unwrap();
    file_path.write_all(html.as_bytes()).unwrap();
}
