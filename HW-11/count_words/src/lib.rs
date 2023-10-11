use std::fs::File;
use std::io::Read;

pub fn count_total_words(text: &str) -> usize {
    let word = text.split(|c: char| c.is_whitespace() || c == ',')
    .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
    .filter(|s| !s.is_empty())
    .collect::<Vec<String>>();
    let count = word.len();
    count
}

pub fn read_to_doc_c(filename: &[String]) -> Vec<(String, usize)> {
    let mut list_docs = Vec::new();
    for file_path in filename {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error opening file: {}", file_path);
                return list_docs;
            }
        };
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let count = count_total_words(&content);
        list_docs.push((file_path.to_string(), count))
    }
    list_docs.sort_by(|a,b| b.1.cmp(&a.1));
    list_docs
}

pub fn generate_html(v: &Vec<(String, usize)>) -> String {
    let mut html = String::from("<style>\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>");
    html.push_str("\n<table>");
    html.push_str("\n\t<tr>\n\t<th>File</th>\n\t<th>Word Count</th>\n\t</tr>");
    
    for (name, count) in v {
        html.push_str("\n\t<tr>");
        html.push_str(&format!("\n\t\t<td>{}</td>\n\t\t<td>{}</td>", name, count));
        html.push_str("\n\t</tr>");
    }
    html.push_str("\n</table>");
    html
}